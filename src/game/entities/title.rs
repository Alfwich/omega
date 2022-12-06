use crate::app::App;
use crate::core::component::audio_clip::AudioClip;
use crate::core::component::component::Component;
use crate::core::component::image::Image;
use crate::core::component::text::Text;
use crate::core::entity::{Entity, EntityFns};
use crate::core::event::{Event, ImageLoadEventPayload};
use crate::core::renderer::renderer::Renderer;
use crate::core::renderer::renderer::Viewport;
use crate::game::entities::button::make_button;

use rand::Rng;
use sfml::window::{Event as SFMLEvent, Key};

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
static DISK_IMAGE_PATH: &str = "res/img/motorcycle.png";

fn update_title(e: &mut Entity, _app: &App, dt: f32) {
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

    {
        let card = e.find_component::<Image>("card").unwrap();
        card.rotation += dt * 4.;
    }

    {
        let beep = e.find_component::<AudioClip>("beep").unwrap();
        if beep.sound.get_sound().status() == sfml::audio::SoundStatus::STOPPED {
            beep.sound.get_sound().play();
        }
    }
    {
        let button = e.find_child_by_name("test_button").unwrap();
        let mut _button_bg = button.find_component::<Image>("background").unwrap();
    }
}

fn handle_event(e: &mut Entity, app: &mut App, ev: &Event) {
    let card = e.find_component::<Image>("card").unwrap();
    match ev {
        Event::SFMLEvent(ev) => match ev {
            SFMLEvent::MouseMoved { x, y } => {
                card.x = *x;
                card.y = *y;
            }
            SFMLEvent::KeyPressed { code, .. } => match code {
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
                &Key::U => {
                    let info = app.resource.load_image_from_disk(DISK_IMAGE_PATH).unwrap();
                    let mut dynamic_cmp = Image::new("");
                    dynamic_cmp.texture_id = Some(info.texture_id);
                    dynamic_cmp.x = rand::thread_rng().gen_range(0..1000);
                    dynamic_cmp.y = rand::thread_rng().gen_range(0..1000);
                    dynamic_cmp.width = info.width;
                    dynamic_cmp.height = info.height;
                    dynamic_cmp.color.x = rand::thread_rng().gen_range(0f32..1f32);
                    dynamic_cmp.color.y = rand::thread_rng().gen_range(0f32..1f32);
                    dynamic_cmp.color.z = rand::thread_rng().gen_range(0f32..1f32);
                    e.add_component(dynamic_cmp);
                }
                _ => {}
            },
            _ => {}
        },
        Event::ImageLoadEvent(ImageLoadEventPayload(url, id, width, height)) => {
            if url == REMOTE_IMAGE_URL {
                card.texture_id = Some(*id);
                card.width = *width;
                card.height = *height;
            } else if url == DISK_IMAGE_PATH {
                let async_local = e.find_component::<Image>("async_local").unwrap();
                async_local.texture_id = Some(*id);
                async_local.width = *width;
                async_local.height = *height;
            }
        }
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

    e.add_component(Data::default());

    {
        let texture_info = app
            .resource
            .load_image_from_disk("res/img/background.png")
            .unwrap();

        let mut image = Image::with_texture(
            "background",
            texture_info.texture_id,
            texture_info.width,
            texture_info.height,
        );
        image.x = (viewport.window_size.0 / 2.) as i32;
        image.y = (viewport.window_size.1 / 2.) as i32;
        e.add_component(image);
    }

    {
        app.resource.load_image_from_disk_async(DISK_IMAGE_PATH);
        let mut async_local = Image::new("async_local");
        async_local.x = 1000;
        async_local.y = 1000;
        e.add_component(async_local);
    }

    {
        app.resource.load_image_from_url_async(REMOTE_IMAGE_URL);
        e.add_component(Image::new("card"));
    }

    {
        let text_texture = app.resource.load_text_texture("Omega Ω").unwrap();
        let mut text = Text::new("title", &text_texture);
        text.x = (viewport.window_size.0 / 2.) as i32;
        text.y = (viewport.window_size.1 / 2.) as i32;
        e.add_component(text);

        let d = 5;
        for x in 1..d {
            for y in 1..d {
                let mut t = Text::new("", &text_texture);
                t.x = x * (viewport.window_size.0 as i32 / d);
                t.y = y * (viewport.window_size.1 as i32 / d);
                e.add_component(t);
            }
        }
    }

    {
        let audio_data = app.resource.load_audio_data("res/snd/beep.wav").unwrap();
        let beep = AudioClip::new("beep", &audio_data);
        e.add_component(beep);
    }

    {
        let mut button = make_button(app, viewport);
        button.name = "test_button".to_string();
        e.add_child(button);
    }

    return e;
}
