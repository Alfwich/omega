use sfml::audio::{Sound as SFMLSound, SoundBuffer};

pub struct Sound {
    name: String,
    sound_buffer: sfml::SfBox<SoundBuffer>,
    pub sound: Option<SFMLSound>,
}

impl Sound {
    pub fn new(name: &str, sound_file: &str) -> Self {
        Sound {
            name: name.to_string(),
            sound_buffer: SoundBuffer::from_file(sound_file).unwrap(),
            sound: None,
        }
    }

    pub fn play(&mut self) {
        match &self.sound {
            None => self.sound = Some(SFMLSound::new(&self.sound_buffer)),
            _ => {}
        }

        match &mut self.sound {
            Some(sound) => {
                sound.play();
            }
            _ => {}
        }
    }
}
