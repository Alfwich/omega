use crate::Entity;
use crate::core::renderer::app_gl;

pub fn make_title() -> Entity {
    let mut e = Entity::new(|e, d| {
        let img : &mut crate::core::component::image::Image = e.components[0].as_any().downcast_mut().unwrap();
        img.rotation += d;
    });
    let texture_id = app_gl::load_image_from_disk("res/img/background.png", 1440, 1070).unwrap();
    let image = crate::core::component::image::Image::new(texture_id, 1440, 1070);
    e.components.push(Box::new(image));
    return e;
}

