use crate::core::image::Image;
use crate::app_gl;

#[derive(Debug)]
pub struct GameState {
    pub background_image: Image,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            background_image: Image {
                texture_id: app_gl::load_image_from_disk("res/img/background.png", 1440, 1070).unwrap(),
                ..Default::default()
            },
        }
    }
}
