use crate::app::App;
use crate::core::component::audio_clip::AudioClip;
use crate::core::component::component::Component;
use crate::core::component::image::Image;
use crate::core::entity::entity::{Entity, EntityFns};
use crate::core::event::{Event, UpdateRenderablePayload};

use sfml::window::Event as SFMLEvent;

use core::any::Any;

#[derive(Default, Debug, Clone, Copy)]
struct Data {
    x: f32,
    y: f32,
    button_down: bool,
}

impl Component for Data {
    fn get_name(&self) -> &str {
        "data"
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

fn update_button(e: &mut Entity, _app: &App, _dt: f32) {
    let data = *e.find_component::<Data>("data").unwrap();
    let over_button;
    {
        let render_offset_x = 0.;
        let render_offset_y = 0.;
        let button = e.find_component::<Image>("background").unwrap();
        let bx = render_offset_x + button.x;
        let by = render_offset_y + button.y;

        let mx = data.x;
        let my = data.y;
        let half_width = button.width / 2.;
        let half_height = button.height / 2.;
        over_button = mx > bx - half_width
            && mx < bx + half_width
            && my > by - half_height
            && my < by + half_height;

        if over_button {
            button.color.x = 0.;
        } else {
            button.color.x = 1.;
        }
    }

    if data.button_down && over_button {
        {
            let beep = e.find_component::<AudioClip>("zombie").unwrap();
            if beep.sound.get_sound().status() == sfml::audio::SoundStatus::STOPPED {
                beep.sound.get_sound().play();
            }
        }
    }
}

fn handle_event(e: &mut Entity, _app: &mut Option<&mut App>, ev: &Event) {
    let data = e.find_component::<Data>("data").unwrap();
    match ev {
        Event::SFMLEvent(sev) => match sev {
            SFMLEvent::MouseMoved { x, y } => {
                data.x = *x as f32;
                data.y = *y as f32;
            }
            SFMLEvent::MouseButtonPressed { button, .. } => match button {
                &sfml::window::mouse::Button::Left => {
                    data.button_down = true;
                }
                _ => {}
            },
            SFMLEvent::MouseButtonReleased { button, .. } => match button {
                &sfml::window::mouse::Button::Left => {
                    data.button_down = false;
                }
                _ => {}
            },
            _ => {}
        },
        Event::UpdateRenderable(p) => {
            let button = e.find_component::<Image>("background").unwrap();
            match p {
                UpdateRenderablePayload::X(x) => {
                    button.x = *x;
                }
                UpdateRenderablePayload::MoveX(mx) => {
                    button.x += *mx;
                }
                UpdateRenderablePayload::Y(y) => {
                    button.y = *y;
                }
                UpdateRenderablePayload::MoveY(my) => {
                    button.y += *my;
                }
                _ => {}
            }
        }
        _ => {}
    }
}

pub fn make_button(app: &mut App) -> Entity {
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
        let bg = Image::with_texture("background", &bg_image, 100., 100.);
        e.add_component(bg);
    }

    {
        let audio_data = app.resource.load_audio_data("res/snd/zombie.wav").unwrap();
        let zombie = AudioClip::new("zombie", audio_data);
        e.add_component(zombie);
    }

    e
}
