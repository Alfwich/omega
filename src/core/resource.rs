use crate::core::renderer::app_gl::*;
use sfml::{audio::SoundBuffer, window::Context, SfBox};
use std::fs::File;

use std::io::Read;
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::{self};
use std::{cell::RefCell, collections::HashMap};

use super::renderer::app_gl;

#[derive(Default, Clone)]
pub enum ImageLoadPayloadType {
    #[default]
    Remote,
    Disk,
}

#[derive(Default, Clone)]
pub struct ImageLoadPayload {
    pub handle: AsyncLoadHandle,
    pub image_type: ImageLoadPayloadType,
    pub path: String,
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct AsyncLoadHandle {
    pub id: u32,
}

pub enum AsyncLoadError {
    ResourceAlreadyExists(u32),
    FailedToCommunicateWithResourceThread,
}

pub struct TextLoadInfo {
    pub text: String,
    pub font_path: String,
    pub font_size: isize,
}

impl Default for TextLoadInfo {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            font_path: "res/font/default.otf".to_string(),
            font_size: 36,
        }
    }
}

struct Font {
    pub data: Rc<Vec<u8>>,
}

pub struct Resources {
    /// Internal repositories for dynamic game data
    audio_data: HashMap<String, RefCell<SfBox<SoundBuffer>>>,
    texture_data: HashMap<String, Texture>,
    text_data: HashMap<String, Texture>,
    font_data: HashMap<String, Font>,

    remote_image_loading: HashMap<String, u32>,
    remote_image_work_tx: Sender<ImageLoadPayload>,
    remote_image_rx: Receiver<ImageLoadPayload>,
    base_handle: AsyncLoadHandle,
}

fn image_loading_proc_thread(rx: Receiver<ImageLoadPayload>, tx: Sender<ImageLoadPayload>) {
    let client = reqwest::blocking::Client::new();
    loop {
        let url_to_load = rx.recv();
        match url_to_load {
            Ok(payload) => {
                if payload.texture_id != 0 {
                    if let Err(_msg) = tx.send(ImageLoadPayload {
                        handle: payload.handle,
                        image_type: payload.image_type,
                        path: payload.path,
                        texture_id: payload.texture_id,
                        width: payload.width,
                        height: payload.height,
                    }) {
                        return;
                    } else {
                        continue;
                    }
                }
                // Create a new GL context for loading this image data as it is needed.
                let _context = Context::new();
                match payload.image_type {
                    ImageLoadPayloadType::Remote => {
                        if let Ok(result) = load_image_from_url(&client, &payload.path) {
                            if let Err(_msg) = tx.send(ImageLoadPayload {
                                handle: payload.handle,
                                image_type: payload.image_type,
                                path: payload.path,
                                texture_id: result.texture_id,
                                width: result.width,
                                height: result.height,
                            }) {
                                return;
                            } else {
                                continue;
                            }
                        }
                    }
                    ImageLoadPayloadType::Disk => {
                        if let Ok(result) = load_image_from_disk(&payload.path) {
                            if let Err(_msg) = tx.send(ImageLoadPayload {
                                handle: payload.handle,
                                image_type: payload.image_type,
                                path: payload.path,
                                texture_id: result.texture_id,
                                width: result.width,
                                height: result.height,
                            }) {
                                return;
                            } else {
                                continue;
                            }
                        }
                    }
                }
            }
            _ => {
                return;
            }
        }
    }
}

impl Default for Resources {
    fn default() -> Self {
        let (in_tx, in_rx) = std::sync::mpsc::channel();
        let (out_tx, out_rx) = std::sync::mpsc::channel();

        // Start detached proc thread for async loading.
        // This proc thread will terminate when the in channel gets droped.
        thread::spawn(move || image_loading_proc_thread(in_rx, out_tx));

        Resources {
            audio_data: HashMap::new(),
            texture_data: HashMap::new(),
            text_data: HashMap::new(),
            font_data: HashMap::new(),
            remote_image_loading: HashMap::new(),
            remote_image_work_tx: in_tx,
            remote_image_rx: out_rx,
            base_handle: AsyncLoadHandle::default(),
        }
    }
}

impl Resources {
    pub fn recv_load_events(&mut self) -> Option<(String, ImageLoadPayload)> {
        if let Ok(payload) = self.remote_image_rx.try_recv() {
            let payload_key = payload.path.clone();
            let ppath = payload.path.clone();

            // Special case where the texture gets loaded sync before an async request resolves
            if self.texture_data.contains_key(&payload_key) {
                if payload.texture_id != 0
                    && payload.texture_id != self.texture_data[&payload_key].texture_id
                {
                    app_gl::release_texture(payload.texture_id);
                }
                return Some((
                    payload_key,
                    ImageLoadPayload {
                        handle: payload.handle,
                        image_type: payload.image_type,
                        path: payload.path.clone(),
                        texture_id: self.texture_data[&ppath].texture_id,
                        width: self.texture_data[&ppath].width,
                        height: self.texture_data[&ppath].height,
                    },
                ));
            } else {
                self.texture_data.insert(
                    payload.path.clone(),
                    Texture {
                        texture_id: payload.texture_id,
                        width: payload.width,
                        height: payload.height,
                    },
                );
                self.remote_image_loading.remove(&ppath);
                return Some((ppath, payload.clone()));
            }
        }

        None
    }

    pub fn load_audio_data(
        &mut self,
        audio_file_path: &str,
    ) -> Result<&RefCell<SfBox<SoundBuffer>>, String> {
        if !self.audio_data.contains_key(audio_file_path) {
            self.audio_data.insert(
                audio_file_path.to_string(),
                RefCell::new(sfml::audio::SoundBuffer::from_file(audio_file_path).unwrap()),
            );
        }
        Ok(&self.audio_data[audio_file_path])
    }

    pub fn load_image_from_disk(&mut self, image_file_path: &str) -> Result<Texture, String> {
        if let Some(id) = self.texture_data.get(&image_file_path.to_string()) {
            Ok(*id)
        } else {
            let id = load_image_from_disk(image_file_path)?;
            self.texture_data.insert(image_file_path.to_string(), id);
            Ok(id)
        }
    }

    pub fn load_image_from_disk_async(
        &mut self,
        image_path: &str,
    ) -> Result<AsyncLoadHandle, AsyncLoadError> {
        let texture_data = *self
            .texture_data
            .get(image_path)
            .unwrap_or(&Texture::default());
        let image_is_loading = self
            .remote_image_loading
            .contains_key(&image_path.to_string());
        if texture_data.texture_id != 0 {
            return Err(AsyncLoadError::ResourceAlreadyExists(
                texture_data.texture_id,
            ));
        } else if image_is_loading {
            return Ok(AsyncLoadHandle {
                id: self.remote_image_loading[image_path],
            });
        };

        self.base_handle.id += 1;

        self.remote_image_loading
            .insert(image_path.to_string(), self.base_handle.id);
        match self.remote_image_work_tx.send(ImageLoadPayload {
            handle: self.base_handle,
            image_type: ImageLoadPayloadType::Disk,
            path: image_path.to_string(),
            texture_id: texture_data.texture_id,
            width: 0,
            height: 0,
        }) {
            Err(msg) => {
                println!("Failed to send async image load request: {:?}", msg);
                Err(AsyncLoadError::FailedToCommunicateWithResourceThread)
            }
            _ => Ok(self.base_handle),
        }
    }

    pub fn load_image_from_url_async(
        &mut self,
        image_url: &str,
    ) -> Result<AsyncLoadHandle, AsyncLoadError> {
        let texture_info = *self
            .texture_data
            .get(image_url)
            .unwrap_or(&Texture::default());
        let remote_image_is_loading = self
            .remote_image_loading
            .contains_key(&image_url.to_string());
        if texture_info.texture_id != 0 {
            return Err(AsyncLoadError::ResourceAlreadyExists(
                texture_info.texture_id,
            ));
        } else if remote_image_is_loading {
            return Ok(AsyncLoadHandle {
                id: self.remote_image_loading[image_url],
            });
        }

        self.base_handle.id += 1;
        self.remote_image_loading
            .insert(image_url.to_string(), self.base_handle.id);
        match self.remote_image_work_tx.send(ImageLoadPayload {
            handle: self.base_handle,
            image_type: ImageLoadPayloadType::Remote,
            path: image_url.to_string(),
            texture_id: texture_info.texture_id,
            width: 0,
            height: 0,
        }) {
            Err(msg) => {
                println!("Failed to send async image load request {:?}", msg);
                Err(AsyncLoadError::FailedToCommunicateWithResourceThread)
            }
            _ => Ok(self.base_handle),
        }
    }

    pub fn load_text_texture(&mut self, text_load_info: &TextLoadInfo) -> Result<Texture, String> {
        let key = text_load_info.text.clone();
        if let Some(id) = self.text_data.get(&key) {
            Ok(*id)
        } else {
            let font_path = text_load_info.font_path.clone();
            self.font_data.entry(font_path.clone()).or_insert_with(|| {
                let mut source = File::open(font_path.clone()).unwrap();
                let mut contents = Vec::new();
                source
                    .read_to_end(&mut contents)
                    .map_err(|err| println!("{:?}", err))
                    .ok();
                Font {
                    data: Rc::new(contents),
                }
            });
            let text_result = render_text_to_texture(RenderTextBundle {
                text: &text_load_info.text,
                text_size: text_load_info.font_size,
                font_data: &self.font_data.get(&font_path).unwrap().data.clone(),
            })?;
            self.text_data.insert(key, text_result);
            Ok(text_result)
        }
    }
}

impl Drop for Resources {
    fn drop(&mut self) {
        for texture_info in self.texture_data.values() {
            release_texture(texture_info.texture_id);
        }

        for texture_info in self.text_data.values() {
            release_texture(texture_info.texture_id);
        }
    }
}
