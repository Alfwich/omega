use crate::core::component::image::Image;
use crate::core::component::text::Text;
use crate::core::renderer::app_gl;
use crate::Entity;

use sfml::window::{Event, Key};

fn update_title(e: &mut Entity, dt: f32) {
    let img = e.find_component::<Image>("background").unwrap();
    img.rotation += dt;
    let title = e.find_component::<Text>("title").unwrap();
    title.rotation -= dt;
}

fn handle_event(e: &mut Entity, ev: &Event) {
    let title = e.find_component::<Text>("title").unwrap();
    match ev {
        Event::KeyPressed { code, .. } => match code {
            &Key::W => {
                title.y += 10;
            }
            &Key::A => {
                title.x -= 10;
            }
            &Key::S => {
                title.y -= 10;
            }
            &Key::D => {
                title.x += 10;
            }
            _ => {}
        },
        _ => {}
    }
}

pub fn make_title() -> Entity {
    let mut e = Entity::new("title", update_title, handle_event);

    let texture_id = app_gl::load_image_from_disk("res/img/background.png", 1440, 1070).unwrap();
    let image = Image::new("background", texture_id, 1920, 1080);
    e.components.push(Box::new(image));
    let text = Text::new("title", "Omega Survival");
    e.components.push(Box::new(text));

    return e;
}
