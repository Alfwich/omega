use crate::core::renderer::app_gl::*;
use sfml::{audio::SoundBuffer, window::Context, SfBox};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

#[derive(Default, Clone)]
pub enum ImageLoadPayloadType {
    #[default]
    Remote,
    Disk,
}

#[derive(Default, Clone)]
pub struct ImageLoadPayload {
    pub image_type: ImageLoadPayloadType,
    pub path: String,
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
}

pub struct Resources {
    pub audio_data: HashMap<String, RefCell<SfBox<SoundBuffer>>>,
    pub texture_data: HashMap<String, ImageResult>,
    pub text_data: HashMap<String, ImageResult>,
    remote_image_loading: HashSet<String>,
    remote_image_work_tx: Sender<ImageLoadPayload>,
    remote_image_rx: Receiver<ImageLoadPayload>,
}

fn image_loading_proc_thread(rx: Receiver<ImageLoadPayload>, tx: Sender<ImageLoadPayload>) {
    let client = reqwest::blocking::Client::new();
    loop {
        let url_to_load = rx.recv();
        match url_to_load {
            Ok(payload) => {
                if payload.texture_id != 0 {
                    if let Err(_msg) = tx.send(ImageLoadPayload {
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
                // HACK: Unsure why the thread-specific context failes to upload texture data inside the rx block.
                //       Create a new GL context for loading this image data as it is needed.
                let _context = Context::new();
                match payload.image_type {
                    ImageLoadPayloadType::Remote => {
                        if let Ok(result) = load_image_from_url(&client, &payload.path) {
                            if let Err(_msg) = tx.send(ImageLoadPayload {
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

        thread::spawn(move || image_loading_proc_thread(in_rx, out_tx));

        Resources {
            audio_data: HashMap::new(),
            texture_data: HashMap::new(),
            text_data: HashMap::new(),
            remote_image_loading: HashSet::new(),
            remote_image_work_tx: in_tx,
            remote_image_rx: out_rx,
        }
    }
}

impl Resources {
    pub fn recv_load_events(&mut self) -> Option<(String, ImageLoadPayload)> {
        match self.remote_image_rx.try_recv() {
            Ok(payload) => {
                self.texture_data.insert(
                    payload.path.clone(),
                    ImageResult {
                        texture_id: payload.texture_id,
                        width: payload.width,
                        height: payload.height,
                    },
                );
                self.remote_image_loading.remove(&payload.path.clone());
                return Some((payload.path.clone(), payload.clone()));
            }
            _ => {}
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

    pub fn load_image_from_disk(&mut self, image_file_path: &str) -> Result<ImageResult, String> {
        if let Some(id) = self.texture_data.get(&image_file_path.to_string()) {
            Ok(*id)
        } else {
            let id = load_image_from_disk(image_file_path)?;
            self.texture_data.insert(image_file_path.to_string(), id);
            Ok(id)
        }
    }

    pub fn load_image_from_disk_async(&mut self, image_path: &str) {
        let texture_data = *self
            .texture_data
            .get(image_path)
            .unwrap_or(&ImageResult::default());
        let image_is_loading = self.remote_image_loading.contains(&image_path.to_string());
        if texture_data.texture_id != 0 || image_is_loading {
            return;
        };

        self.remote_image_loading.insert(image_path.to_string());
        if let Err(_msg) = self.remote_image_work_tx.send(ImageLoadPayload {
            image_type: ImageLoadPayloadType::Disk,
            path: image_path.to_string(),
            texture_id: texture_data.texture_id,
            width: 0,
            height: 0,
        }) {
            print!("Failed to send async image load request");
        }
    }

    pub fn load_image_from_url_async(&mut self, image_url: &str) {
        let texture_info = *self
            .texture_data
            .get(image_url)
            .unwrap_or(&ImageResult::default());
        let remote_image_is_loading = self.remote_image_loading.contains(&image_url.to_string());
        if texture_info.texture_id != 0 || remote_image_is_loading {
            return;
        };

        self.remote_image_loading.insert(image_url.to_string());
        if let Err(_msg) = self.remote_image_work_tx.send(ImageLoadPayload {
            image_type: ImageLoadPayloadType::Remote,
            path: image_url.to_string(),
            texture_id: texture_info.texture_id,
            width: 0,
            height: 0,
        }) {
            print!("Failed to send async image load request");
        }
    }

    pub fn load_text_texture(&mut self, text: &str) -> Result<ImageResult, String> {
        if let Some(id) = self.text_data.get(&text.to_string()) {
            return Ok(*id);
        } else {
            let text_result = render_text_to_texture(text)?;
            self.text_data.insert(text.to_string(), text_result);
            return Ok(text_result);
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
        self.texture_data.clear();
        self.audio_data.clear();
        self.text_data.clear();
    }
}
