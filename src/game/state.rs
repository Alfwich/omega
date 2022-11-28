use crate::app_gl;
use crate::core::component::image::Image;

#[derive(Debug)]
pub struct GameState {
}

impl Default for GameState {
    fn default() -> Self {
        /*
        let title_text = app_gl::render_text_to_texture("Omega Survival").unwrap();

        let client = reqwest::blocking::Client::new();
        let remote_image_id = app_gl::load_image_from_url(
            &client,
            "http://wuteri.ch/misc/visualguider/image/card/Teleport.jpg",
        )
        .unwrap();
        */

        GameState {
        }
    }
}
