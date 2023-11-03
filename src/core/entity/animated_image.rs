use crate::app::App;
use crate::core::component::component::Component;
use crate::core::component::image::{Image, ImageRenderType};
use crate::core::entity::entity::{Entity, EntityFns};
use crate::core::event::Event;
use crate::core::renderer::renderer::Renderer;
use crate::util::rect::Rect;

use core::any::Any;
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
struct Data {
    timer: u64,
    fps_mult: f32,
    frame: usize,
    frames: Vec<Rect>,
    frame_range: (usize, usize),
    animations: HashMap<String, (usize, usize)>,
}

impl Component for Data {
    fn get_name(&self) -> &str {
        "data"
    }

    fn render(&self, _renderer: &Renderer) {}

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

fn update_animated_image(e: &mut Entity, _app: &App, dt: f32) {
    let data;
    {
        let d = e.find_component::<Data>("data").unwrap();
        let range = d.frame_range.1 - d.frame_range.0;
        d.timer += (dt * 1000. * d.fps_mult) as u64;
        let pos = match range {
            0 => 0,
            _ => (d.timer / 1000) as usize % range,
        };
        d.frame = d.frame_range.0 + pos;
        data = d.clone();
    }

    {
        let img = e.find_component::<Image>("ai-texture").unwrap();
        img.r_rect = Some(data.frames[data.frame].clone());
    }
}

fn handle_event(e: &mut Entity, _app: &mut Option<&mut App>, ev: &Event) {
    match ev {
        Event::UpdateRenderable(p) => {
            let img = e.find_component::<Image>("ai-texture").unwrap();

            if let Some(x) = p.x {
                img.x = x;
            }

            if let Some(y) = p.y {
                img.y = y;
            }

            if let Some(mx) = p.move_x {
                img.x += mx;
            }

            if let Some(my) = p.move_y {
                img.y += my;
            }

            if let Some(w) = p.w {
                img.width = w;
            }

            if let Some(h) = p.h {
                img.height = h;
            }

            if let Some(r) = p.r {
                img.rotation = r;
            }

            if let Some(sx) = p.scale_x {
                img.scale.0 = sx;
            }

            if let Some(sy) = p.scale_y {
                img.scale.1 = sy;
            }
        }
        _ => {}
    }
}

pub fn animated_image_set_animation(e: &mut Entity, name: &str) {
    let d = e.find_component::<Data>("data").unwrap();
    if d.animations.contains_key(name) {
        d.frame_range = d.animations[name];
        d.frame = 0;
    }
}
pub fn animated_image_add_animation(e: &mut Entity, name: &str, frame_range: (usize, usize)) {
    let d = e.find_component::<Data>("data").unwrap();
    d.animations.insert(name.to_string(), frame_range);
}

pub fn make_animated_image(
    app: &mut App,
    name: &str,
    texture_name: &str,

    // Width/Height of the cell
    width: f32,
    height: f32,
    fps: Option<f32>,
    image_render_type: Option<ImageRenderType>,
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
        while (y_pos + height as u32) < texture_info.height {
            while (x_pos + width as u32) < texture_info.width {
                d.frames.push(Rect {
                    x: x_pos as f32,
                    y: y_pos as f32,
                    w: width,
                    h: height,
                });
                x_pos += width as u32;
            }
            x_pos = 0;
            y_pos += height as u32;
        }

        d.frame_range = (0, d.frames.len());

        d.fps_mult = match fps {
            Some(f) => f,
            _ => 1.,
        };

        e.add_component(d);
    }

    {
        let mut img = Image::with_texture("ai-texture", &texture_info, width, height);
        img.r_rect = Some(Rect {
            x: 0.,
            y: 0.,
            w: width,
            h: height,
        });

        if image_render_type.is_some() {
            img.render_type = image_render_type;
        }

        e.add_component(img);
    }

    e
}
