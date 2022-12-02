use crate::core::audio::sound::Sound;
use crate::core::component::component::Component;

use core::any::Any;

pub struct AudioClip {
    pub name: String,
    pub sound: Sound,
}

impl AudioClip {
    pub fn new(name: &str, sound_path: &str) -> Self {
        AudioClip {
            name: name.to_string(),
            sound: Sound::new(sound_path),
        }
    }
}

impl Component for AudioClip {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
