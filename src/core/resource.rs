use crate::core::renderer::app_gl::*;
use sfml::{audio::SoundBuffer, SfBox};
use std::{cell::RefCell, collections::HashMap};

#[derive(Default)]
pub struct Resources {
    pub audio_data: HashMap<String, RefCell<SfBox<SoundBuffer>>>,
    pub texture_data: HashMap<String, u32>,
    pub text_data: HashMap<String, TextImageResult>,
}

impl Resources {
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

    pub fn load_image_from_url(&mut self, image_url: &str) -> Result<u32, String> {
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
