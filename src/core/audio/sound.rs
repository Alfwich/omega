use sfml::audio::{Sound as SFMLSound, SoundBuffer};

pub struct Sound {
    name: String,
    sound_buffer: sfml::SfBox<SoundBuffer>,
    sound: Option<SFMLSound>,
}

impl Sound {
    pub fn new(name: &str, sound_file: &str) -> Self {
        let mut result = Sound {
            name: name.to_string(),
            sound_buffer: SoundBuffer::from_file(sound_file).unwrap(),
            sound: None,
        };
        result.sound = Some(SFMLSound::new(&result.sound_buffer));
        return result;
    }

    pub fn get_sound(&mut self) -> &mut SFMLSound {
        self.sound.as_mut().unwrap()
    }
}
