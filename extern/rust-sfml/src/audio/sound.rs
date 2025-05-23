use crate::{
    audio::{SoundBuffer, SoundSource, SoundStatus},
    ffi,
    system::{Time, Vector3f},
    SfBox,
};
use std::ptr::NonNull;

/// Regular sound that can be played in the audio environment.
///
/// `Sound` is the type to use to play sounds.
///
/// It provides:
///
/// - Control (play, pause, stop)
/// - Ability to modify output parameters in real-time (pitch, volume, ...)
/// - 3D spatial features (position, attenuation, ...).
///
/// `Sound` is perfect for playing short sounds that can fit in memory and require no latency,
/// like foot steps or gun shots. For longer sounds, like background musics or long speeches,
/// rather see [`Music`] (which is based on streaming).
///
/// In order to work, a sound must be given a buffer of audio data to play.
/// Audio data (samples) is stored in [`SoundBuffer`], and attached to a sound with the
/// [`set_buffer`] function. The buffer object attached to a sound must remain alive as long as
/// the sound uses it. Note that multiple sounds can use the same sound buffer at the same time.
///
/// [`set_buffer`]: Sound::set_buffer
///
/// # Usage example
///
/// ```no_run
/// use sfml::audio::{Sound, SoundBuffer};
///
/// let buffer = SoundBuffer::from_file("sound.wav").unwrap();
/// let mut sound = Sound::with_buffer(&buffer);
/// sound.play();
/// ```
///
/// [`Music`]: crate::audio::Music
#[derive(Debug)]
pub struct Sound {
    sound: NonNull<ffi::audio::sfSound>,
}

impl Sound {
    /// Create a new `Sound`
    #[must_use]
    pub fn new(buffer: &SfBox<SoundBuffer>) -> Sound {
        let s = unsafe { ffi::audio::sfSound_create() };
        let result = Sound {
            sound: NonNull::new(s).expect("Failed to create Sound"),
        };

        unsafe { ffi::audio::sfSound_setBuffer(s, &**buffer) }

        return result;
    }

    /// Sets whether this sound should loop or not.
    pub fn set_looping(&mut self, looping: bool) {
        unsafe { ffi::audio::sfSound_setLoop(self.sound.as_ptr(), looping) }
    }

    /// Tell whether or not a sound is in loop mode
    ///
    /// Return true if the sound is looping, false otherwise
    #[must_use]
    pub fn is_looping(&self) -> bool {
        unsafe { ffi::audio::sfSound_getLoop(self.sound.as_ptr()) }
    }

    /// Start or resume playing a sound
    ///
    /// This function starts the sound if it was stopped, resumes
    /// it if it was paused, and restarts it from beginning if it
    /// was it already playing.
    /// This function uses its own thread so that it doesn't block
    /// the rest of the program while the sound is played.
    pub fn play(&mut self) {
        unsafe { ffi::audio::sfSound_play(self.sound.as_ptr()) }
    }

    /// Pause a sound
    ///
    /// This function pauses the sound if it was playing,
    /// otherwise (sound already paused or stopped) it has no effect.
    pub fn pause(&mut self) {
        unsafe { ffi::audio::sfSound_pause(self.sound.as_ptr()) }
    }

    /// Stop playing a sound
    ///
    /// This function stops the sound if it was playing or paused,
    /// and does nothing if it was already stopped.
    /// It also resets the playing position (unlike pause).
    pub fn stop(&mut self) {
        unsafe { ffi::audio::sfSound_stop(self.sound.as_ptr()) }
    }

    /// Get the current status of a sound (stopped, paused, playing)
    ///
    /// Return current status
    #[must_use]
    pub fn status(&self) -> SoundStatus {
        unsafe { SoundStatus(ffi::audio::sfSound_getStatus(self.sound.as_ptr())) }
    }

    /// Get the current playing position of a sound
    ///
    /// Return the current playing position
    #[must_use]
    pub fn playing_offset(&self) -> Time {
        unsafe { Time::from_raw(ffi::audio::sfSound_getPlayingOffset(self.sound.as_ptr())) }
    }

    /// Change the current playing position of a sound
    ///
    /// The playing position can be changed when the sound is
    /// either paused or playing.
    ///
    /// # Arguments
    /// * timeOffset - New playing position
    pub fn set_playing_offset(&mut self, time_offset: Time) {
        unsafe { ffi::audio::sfSound_setPlayingOffset(self.sound.as_ptr(), time_offset.raw()) }
    }

    /// Set the source buffer containing the audio data to play
    ///
    /// # Arguments
    /// * buffer - Sound buffer to attach to the sound
    pub fn set_buffer(&mut self, buffer: &SoundBuffer) {
        unsafe { ffi::audio::sfSound_setBuffer(self.sound.as_ptr(), buffer) }
    }

    /// Get the audio buffer attached to a sound
    ///
    /// Return an option to Sound buffer attached to the sound or None
    #[must_use]
    pub fn buffer(&self) -> Option<&SoundBuffer> {
        unsafe { ffi::audio::sfSound_getBuffer(self.sound.as_ptr()).as_ref() }
    }
}

impl Clone for Sound {
    fn clone(&self) -> Self {
        let s = unsafe { ffi::audio::sfSound_copy(self.sound.as_ptr()) };
        Sound {
            sound: NonNull::new(s).expect("Failed to copy Sound"),
        }
    }
}

impl SoundSource for Sound {
    fn set_pitch(&mut self, pitch: f32) {
        unsafe { ffi::audio::sfSound_setPitch(self.sound.as_ptr(), pitch) }
    }
    fn set_volume(&mut self, volume: f32) {
        unsafe { ffi::audio::sfSound_setVolume(self.sound.as_ptr(), volume) }
    }
    fn set_position<P: Into<Vector3f>>(&mut self, position: P) {
        unsafe { ffi::audio::sfSound_setPosition(self.sound.as_ptr(), position.into()) }
    }
    fn set_relative_to_listener(&mut self, relative: bool) {
        unsafe { ffi::audio::sfSound_setRelativeToListener(self.sound.as_ptr(), relative) }
    }
    fn set_min_distance(&mut self, distance: f32) {
        unsafe { ffi::audio::sfSound_setMinDistance(self.sound.as_ptr(), distance) }
    }
    fn set_attenuation(&mut self, attenuation: f32) {
        unsafe { ffi::audio::sfSound_setAttenuation(self.sound.as_ptr(), attenuation) }
    }
    fn pitch(&self) -> f32 {
        unsafe { ffi::audio::sfSound_getPitch(self.sound.as_ptr()) }
    }
    fn volume(&self) -> f32 {
        unsafe { ffi::audio::sfSound_getVolume(self.sound.as_ptr()) }
    }
    fn position(&self) -> Vector3f {
        unsafe { ffi::audio::sfSound_getPosition(self.sound.as_ptr()) }
    }
    fn is_relative_to_listener(&self) -> bool {
        unsafe { ffi::audio::sfSound_isRelativeToListener(self.sound.as_ptr()) }
    }
    fn min_distance(&self) -> f32 {
        unsafe { ffi::audio::sfSound_getMinDistance(self.sound.as_ptr()) }
    }
    fn attenuation(&self) -> f32 {
        unsafe { ffi::audio::sfSound_getAttenuation(self.sound.as_ptr()) }
    }
}

impl Drop for Sound {
    fn drop(&mut self) {
        unsafe {
            ffi::audio::sfSound_destroy(self.sound.as_ptr());
        }
    }
}
