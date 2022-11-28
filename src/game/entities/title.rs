use crate::core::component::image::Image;
use crate::core::component::text::Text;
use crate::core::renderer::app_gl;
use crate::Entity;

pub fn make_title() -> Entity {
    let mut e = Entity::new("title", |e, d| {
        let img = e.find_component::<Image>("background").unwrap();
        img.rotation += d;
        let title = e.find_component::<Text>("title").unwrap();
        title.rotation -= d;
    });
    let texture_id = app_gl::load_image_from_disk("res/img/background.png", 1440, 1070).unwrap();
    let image = Image::new("background", texture_id, 1920, 1080);
    e.components.push(Box::new(image));
    let text = Text::new("title", "Omega Survival");
    e.components.push(Box::new(text));
    return e;
}
