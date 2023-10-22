use crate::app::App;
use crate::core::component::audio_clip::AudioClip;
use crate::core::component::component::Component;
use crate::core::component::image::{Image, ImageRenderRect};
use crate::core::component::text;
use crate::core::entity::entity::{Entity, EntityFns};
use crate::core::event::Event;
use crate::core::renderer::renderer::Renderer;
use crate::core::renderer::renderer::Viewport;

use sfml::window::Event as SFMLEvent;

use core::any::Any;

#[derive(Default, Debug, Clone)]
struct Data {
    frame: u32,
    frames: Vec<ImageRenderRect>,
}

impl Component for Data {
    fn get_name(&self) -> &str {
        return "data";
    }

    fn render(&self, _renderer: &Renderer) {}

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

fn update_animated_image(e: &mut Entity, _app: &App, _dt: f32) {
    let _data = e.find_component::<Data>("data").unwrap();
}

fn handle_event(e: &mut Entity, _app: &mut App, ev: &Event) {
    let _data = e.find_component::<Data>("data").unwrap();
    match ev {
        _ => {}
    }
}

pub fn make_animated_image(
    app: &mut App,
    name: &str,
    texture_name: &str,

    // Width/Height of the cell
    width: f32,
    height: f32,
) -> Entity {
    let mut e = Entity::new(
        name,
        EntityFns {
            update_fn: update_animated_image,
            event_fn: handle_event,
        },
    );

    let texture_info = app.resource.load_image_from_disk(texture_name).unwrap();

    // Create frames
    let mut x_pos = 0;
    let mut y_pos = 0;

    {
        let mut d = Data::default();
        while y_pos < texture_info.height {
            while x_pos < texture_info.width {
                d.frames.push(ImageRenderRect {
                    x: x_pos as f32,
                    y: y_pos as f32,
                    w: width,
                    h: height,
                });
                x_pos += (width as u32);
            }
            x_pos = 0;
            y_pos += (height as u32);
        }

        e.add_component(d);
    }

    {
        let mut img = Image::with_texture("ai_tex", texture_info.texture_id, width, height);
        img.r_rect = Some(ImageRenderRect {
            x: 0.,
            y: 0.,
            w: width,
            h: height,
        });
        e.add_component(img);
    }

    return e;
}
