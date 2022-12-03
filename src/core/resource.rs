use crate::core::renderer::app_gl::*;
use sfml::{audio::SoundBuffer, window::Context, SfBox};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::{cell::RefCell, collections::HashMap};

struct RemoteImageLoadPayload {
    pub url: String,
    pub texture_id: u32,
}

pub struct Resources {
    pub audio_data: HashMap<String, RefCell<SfBox<SoundBuffer>>>,
    pub texture_data: HashMap<String, u32>,
    pub text_data: HashMap<String, TextImageResult>,
    image_work_tx: Sender<RemoteImageLoadPayload>,
    image_rx: Receiver<RemoteImageLoadPayload>,
}

fn image_loading_proc_thread(
    rx: Receiver<RemoteImageLoadPayload>,
    tx: Sender<RemoteImageLoadPayload>,
) {
    let client = reqwest::blocking::Client::new();
    let _context = Context::new(); // OpenGL context required for loading image data on this thread
    loop {
        let url_to_load = rx.recv();
        match url_to_load {
            Ok(payload) => {
                if let Ok(tid) = load_image_from_url(&client, &payload.url) {
                    println!("Loaded image! {}", payload.url);
                    if let Err(_msg) = tx.send(RemoteImageLoadPayload {
                        url: payload.url,
                        texture_id: tid,
                    }) {
                        return;
                    }
                }
            }
            Err(_) => {
                return;
            }
        }
    }
}

impl Default for Resources {
    fn default() -> Self {
        let (in_tx, in_rx) = mpsc::channel();
        let (out_tx, out_rx) = mpsc::channel();

        thread::spawn(move || image_loading_proc_thread(in_rx, out_tx));

        Resources {
            audio_data: HashMap::new(),
            texture_data: HashMap::new(),
            text_data: HashMap::new(),
            image_work_tx: in_tx,
            image_rx: out_rx,
        }
    }
}

impl Resources {
    pub fn recv_load_events(&mut self) -> Option<(String, u32)> {
        match self.image_rx.try_recv() {
            Ok(payload) => {
                let new_str = payload.url.clone();
                self.texture_data.insert(new_str, payload.texture_id);
                return Some((payload.url.clone(), payload.texture_id));
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

    pub fn load_image_from_disk(&mut self, image_file_path: &str) -> Result<u32, String> {
        let id = load_image_from_disk(image_file_path)?;
        Ok(id)
    }

    pub fn load_image_from_url_async(&self, image_url: &str) {
        if let Err(_msg) = self.image_work_tx.send(RemoteImageLoadPayload {
            url: image_url.to_string(),
            texture_id: 0,
        }) {
            print!("Failed to send async image load request");
        }
    }

    pub fn load_image_from_url(&self, image_url: &str) -> Result<u32, String> {
        let client = reqwest::blocking::Client::new();
        let remote_image_id = load_image_from_url(&client, image_url)?;
        Ok(remote_image_id)
    }

    pub fn load_text_texture(&mut self, text: &str) -> Result<TextImageResult, String> {
        let text = render_text_to_texture(text)?;
        Ok(text)
    }
}

impl Drop for Resources {
    fn drop(&mut self) {
        for texture_id in self.texture_data.values() {
            release_texture(*texture_id);
        }

        for texture_id in self.text_data.values() {
            release_texture(texture_id.texture_id);
        }
        self.texture_data.clear();
        self.audio_data.clear();
        self.text_data.clear();
    }
}
