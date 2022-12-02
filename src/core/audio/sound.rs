use sfml::audio::{Sound as SFMLSound, SoundBuffer};

pub struct Sound {
    sound: SFMLSound,
}

impl Sound {
    pub fn new(buffer: &std::cell::RefCell<sfml::SfBox<SoundBuffer>>) -> Self {
        let sound_buffer = buffer.borrow_mut();
        Sound {
            sound: SFMLSound::new(&sound_buffer),
        }
    }

    pub fn get_sound(&mut self) -> &mut SFMLSound {
        &mut self.sound
    }
}
