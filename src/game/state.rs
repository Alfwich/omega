use crate::app_gl;
use crate::core::component::image::Image;

#[derive(Debug)]
pub struct GameState {
    pub background_image: Image,
    pub title_text: Image,
    pub remote_image: Image,
}

impl Default for GameState {
    fn default() -> Self {
        let title_text = app_gl::render_text_to_texture("Omega Survival").unwrap();

        let client = reqwest::blocking::Client::new();
        let remote_image_id = app_gl::load_image_from_url(
            &client,
            "http://wuteri.ch/misc/visualguider/image/card/Teleport.jpg",
        )
        .unwrap();

        GameState {
            background_image: Image {
                texture_id: app_gl::load_image_from_disk("res/img/background.png", 1440, 1070)
                    .unwrap(),
                ..Default::default()
            },
            title_text: Image {
                texture_id: title_text.texture_id,
                width: title_text.width,
                height: title_text.height,
                ..Default::default()
            },
            remote_image: Image {
                texture_id: remote_image_id,
                width: 223,
                height: 310,
                ..Default::default()
            },
        }
    }
}

impl Drop for GameState {
    fn drop(&mut self) {
        app_gl::release_texture(self.background_image.texture_id);
        app_gl::release_texture(self.title_text.texture_id);
    }
}
