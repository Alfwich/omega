use crate::app::App;
use crate::core::component::component::Component;
use crate::core::component::image::Image;
use crate::core::entity::{Entity, EntityFns};
use crate::core::event::Event;
use crate::core::renderer::renderer::Renderer;
use crate::core::renderer::renderer::Viewport;

use sfml::window::Event as SFMLEvent;

use core::any::Any;

#[derive(Default, Debug, Clone, Copy)]
struct Data {}

impl Component for Data {
    fn get_name(&self) -> &str {
        return "data";
    }

    fn render(&self, _renderer: &Renderer) {}

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

fn update_button(_e: &mut Entity, _app: &App, _dt: f32) {}
fn handle_event(e: &mut Entity, _app: &mut App, ev: &Event) {
    let button = e.find_component::<Image>("background").unwrap();
    match ev {
        Event::SFMLEvent(sev) => match sev {
            SFMLEvent::MouseMoved { x, y } => {
                let mx = *x as f32;
                let my = *y as f32;
                let half_width = button.width / 2.;
                let half_height = button.height / 2.;

                if mx > button.x - half_width
                    && mx < button.x + half_width
                    && my > button.y - half_height
                    && my < button.y + half_height
                {
                    button.color.x = 0.;
                } else {
                    button.color.x = 1.;
                }
            }
            _ => {}
        },
        _ => {}
    }
}

pub fn make_button(app: &mut App, _viewport: &Viewport) -> Entity {
    let mut e = Entity::new(
        "button",
        EntityFns {
            update_fn: update_button,
            event_fn: handle_event,
        },
    );

    e.add_component(Data::default());

    {
        let bg_image = app
            .resource
            .load_image_from_disk("res/img/button.png")
            .unwrap();
        let mut bg = Image::with_texture("background", bg_image.texture_id, 100., 100.);
        e.add_component(bg);
    }

    return e;
}
