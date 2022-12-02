mod app;
mod core;
mod game;
mod util;

fn main() {
    /*
    use crate::core::component::audio_clip::AudioClip;
    use std::cell::RefCell;
    let audio_data_cell =
        RefCell::new(sfml::audio::SoundBuffer::from_file("res/snd/beep.wav").unwrap());

    let mut audio_clip2 = AudioClip::new("beep", &audio_data_cell);
    {
        let mut audio_clip = AudioClip::new("beep", &audio_data_cell);
        audio_clip.sound.get_sound().play();
        while (audio_clip.sound.get_sound().status() == sfml::audio::SoundStatus::PLAYING) {}
    }

    {
        audio_clip2.sound.get_sound().play();
        while (audio_clip2.sound.get_sound().status() == sfml::audio::SoundStatus::PLAYING) {}
    }

    {
        let mut audio_clip = AudioClip::new("beep", &audio_data_cell);
        audio_clip.sound.get_sound().play();
        while (audio_clip.sound.get_sound().status() == sfml::audio::SoundStatus::PLAYING) {}
    }
    */

    app::run();
}
