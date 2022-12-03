use crate::app::App;
use crate::core::component::audio_clip::AudioClip;
use crate::core::component::component::Component;
use crate::core::component::image::Image;
use crate::core::component::text::Text;
use crate::core::entity::{Entity, EntityFns};
use crate::core::renderer::renderer::Renderer;
use crate::core::renderer::renderer::Viewport;

use sfml::window::{Event, Key};

use core::any::Any;

#[derive(Default, Debug, Clone, Copy)]
struct Data {
    counter: f32,
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

static REMOTE_IMAGE_URL: &str = "http://wuteri.ch/misc/visualguider/image/card/Teleport.jpg";

fn update_title(e: &mut Entity, dt: f32) {
    let d;
    {
        let data = e.find_component::<Data>("data").unwrap();
        data.counter += dt;
        d = data.clone();
    }

    {
        let img = e.find_component::<Image>("background").unwrap();
        img.rotation = d.counter.sin();
    }

    {
        let title = e.find_component::<Text>("title").unwrap();
        title.rotation -= dt;
    }

    let beep = e.find_component::<AudioClip>("beep").unwrap();
    if beep.sound.get_sound().status() == sfml::audio::SoundStatus::STOPPED {
        beep.sound.get_sound().play();
    }
}

fn handle_event(e: &mut Entity, ev: &Event) {
    let card = e.find_component::<Image>("card").unwrap();
    match ev {
        Event::MouseMoved { x, y } => {
            card.x = *x;
            card.y = *y;
        }
        Event::KeyPressed { code, .. } => match code {
            &Key::W => {
                card.y -= 10;
            }
            &Key::A => {
                card.x -= 10;
            }
            &Key::S => {
                card.y += 10;
            }
            &Key::D => {
                card.x += 10;
            }
            _ => {}
        },
        _ => {}
    }
}

pub fn make_title(app: &mut App, viewport: &Viewport) -> Entity {
    let mut e = Entity::new(
        "title",
        EntityFns {
            update_fn: update_title,
            event_fn: handle_event,
        },
    );

    let d = Data::default();
    e.components.push(Box::new(d));

    let texture_id = app
        .resource
        .load_image_from_disk("res/img/background.png")
        .unwrap();
    let mut image = Image::with_texture("background", texture_id, 1920, 1080);
    image.x = (viewport.window_size.0 / 2.) as i32;
    image.y = (viewport.window_size.1 / 2.) as i32;
    e.components.push(Box::new(image));

    app.resource.load_image_from_url_async(REMOTE_IMAGE_URL);
    let card_image = Image::new("card");
    e.components.push(Box::new(card_image));

    let text_texture = app.resource.load_text_texture("Omega Ω").unwrap();
    let mut text = Text::new("title", &text_texture);
    text.x = (viewport.window_size.0 / 2.) as i32;
    text.y = (viewport.window_size.1 / 2.) as i32;
    e.components.push(Box::new(text));

    let d = 10;
    for x in 1..d {
        for y in 1..d {
            let mut t = Text::new("", &text_texture);
            t.x = x * (viewport.window_size.0 as i32 / d);
            t.y = y * (viewport.window_size.1 as i32 / d);
            e.components.push(Box::new(t));
        }
    }

    let audio_data = app.resource.load_audio_data("res/snd/beep.wav").unwrap();
    let beep = AudioClip::new("beep", &audio_data);
    e.components.push(Box::new(beep));

    return e;
}
