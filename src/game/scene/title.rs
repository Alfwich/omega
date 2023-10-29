use crate::app::App;
use crate::core::component::audio_clip::AudioClip;
use crate::core::component::component::Component;
use crate::core::component::image::{Image, ImageRenderRect};
use crate::core::component::text::Text;
use crate::core::entity::animated_image::{
    animated_image_add_animation, animated_image_set_animation, animated_image_set_scale,
    make_animated_image,
};
use crate::core::entity::entity::{Entity, EntityFns};
use crate::core::event::Event;
use crate::core::renderer::renderer::Renderer;
use crate::core::renderer::renderer::Viewport;
use crate::game::entity::button::make_button;

use rand::Rng;
use sfml::window::{Event as SFMLEvent, Key};

use core::any::Any;
use std::f32::consts::PI;

#[derive(Default, Debug, Clone, Copy)]
struct Data {
    counter: f32,
    left_down: bool,
    right_down: bool,
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

static REMOTE_IMAGE_URL: &str = "http://wuteri.ch/img/Teleport.jpg";
static DISK_IMAGE_PATH: &str = "res/img/motorcycle.png";
static DISK_IMAGE_QUAD: &str = "res/img/test-clip.png";
static DISK_IMAGE_MARIO: &str = "res/img/mario.png";

fn update_title(e: &mut Entity, _app: &App, dt: f32) {
    let d;
    {
        let data = e.find_component::<Data>("data").unwrap();
        data.counter += dt;
        d = *data;
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
        button.x = d.counter.cos() * 50. * PI * 2. + 300.;
        button.y = d.counter.sin() * 50. * PI * 2. + 300.;
    }
    {
        let test_quad = e.find_component::<Image>("test-quad").unwrap();
        let idx = (d.counter as u32) % 4;
        let new_rect = match idx {
            0 => ImageRenderRect {
                x: 0.,
                y: 0.,
                w: 256.,
                h: 256.,
            },
            1 => ImageRenderRect {
                x: 256.,
                y: 0.,
                w: 256.,
                h: 256.,
            },
            2 => ImageRenderRect {
                x: 0.,
                y: 256.,
                w: 256.,
                h: 256.,
            },
            3 => ImageRenderRect {
                x: 256.,
                y: 256.,
                w: 256.,
                h: 256.,
            },
            _ => ImageRenderRect::default(),
        };
        test_quad.r_rect = Some(new_rect);
    }
    {
        let animated_image = e.find_child_by_name("test-animated").unwrap();
        match (d.left_down, d.right_down) {
            (true, false) => {
                animated_image.x -= 100. * dt;
            }
            (false, true) => {
                animated_image.x += 100. * dt;
            }
            _ => {}
        }
    }
}

fn handle_event(e: &mut Entity, app: &mut App, ev: &Event) {
    let card = e.find_component::<Image>("card").unwrap();
    match ev {
        Event::SFMLEvent(ev) => match ev {
            SFMLEvent::MouseMoved { x, y } => {
                card.x = *x as f32;
                card.y = *y as f32;
            }
            SFMLEvent::KeyPressed { code, .. } => match code {
                &Key::W => {
                    card.y -= 10.;
                }
                &Key::A => {
                    card.x -= 10.;

                    {
                        let animated_image = e.find_child_by_name("test-animated").unwrap();
                        animated_image_set_animation(animated_image, "walking");
                        animated_image_set_scale(animated_image, (-3., 3.));
                    }
                    {
                        let data = e.find_component::<Data>("data").unwrap();
                        data.left_down = true;
                    }
                }
                &Key::S => {
                    card.y += 10.;

                    {
                        let animated_image = e.find_child_by_name("test-animated").unwrap();
                        animated_image_set_animation(animated_image, "swim");
                    }
                }
                &Key::D => {
                    card.x += 10.;

                    {
                        let animated_image = e.find_child_by_name("test-animated").unwrap();
                        animated_image_set_animation(animated_image, "walking");
                        animated_image_set_scale(animated_image, (3., 3.));
                    }
                    {
                        let data = e.find_component::<Data>("data").unwrap();
                        data.right_down = true;
                    }
                }
                &Key::U => {
                    let info = app.resource.load_image_from_disk(DISK_IMAGE_PATH).unwrap();
                    let mut dynamic_cmp = Image::new_nameless();
                    let mut thread_rng = rand::thread_rng();
                    dynamic_cmp.texture = Some(info);
                    dynamic_cmp.x = thread_rng.gen_range(0f32..1000f32);
                    dynamic_cmp.y = thread_rng.gen_range(0f32..1000f32);
                    dynamic_cmp.width = info.width as f32;
                    dynamic_cmp.height = info.height as f32;
                    dynamic_cmp.color.x = thread_rng.gen_range(0f32..1f32);
                    dynamic_cmp.color.y = thread_rng.gen_range(0f32..1f32);
                    dynamic_cmp.color.z = thread_rng.gen_range(0f32..1f32);
                    e.add_component(dynamic_cmp);
                }
                _ => {}
            },
            SFMLEvent::KeyReleased { code, .. } => match code {
                &Key::A => {
                    {
                        let animated_image = e.find_child_by_name("test-animated").unwrap();
                        animated_image_set_animation(animated_image, "idle");
                    }

                    {
                        let data = e.find_component::<Data>("data").unwrap();
                        data.left_down = false;
                    }
                }
                &Key::D => {
                    {
                        let animated_image = e.find_child_by_name("test-animated").unwrap();
                        animated_image_set_animation(animated_image, "idle");
                    }

                    {
                        let data = e.find_component::<Data>("data").unwrap();
                        data.right_down = false;
                    }
                }
                _ => {}
            },
            _ => {}
        },
        Event::ImageLoadEvent(img_data) => {
            if img_data.url == REMOTE_IMAGE_URL {
                card.apply_image(img_data);
            } else if img_data.url == DISK_IMAGE_PATH {
                let async_local = e.find_component::<Image>("async_local").unwrap();
                async_local.apply_image(img_data)
            } else if img_data.url == DISK_IMAGE_QUAD {
                let quad_cmp = e.find_component::<Image>("test-quad").unwrap();
                quad_cmp.apply_image(img_data)
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
            &texture_info,
            texture_info.width as f32,
            texture_info.height as f32,
        );
        image.x = viewport.window_size.0 / 2.;
        image.y = viewport.window_size.1 / 2.;
        e.add_component(image);
    }

    {
        app.resource.load_image_from_disk_async(DISK_IMAGE_PATH);
        let mut async_local = Image::new("async_local");
        async_local.x = 1000.;
        async_local.y = 1000.;
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

        let d = 3;
        for x in 1..d {
            for y in 1..d {
                let i_texture = app
                    .resource
                    .load_text_texture(&format!("Omega {:?}:{:?}", x, y))
                    .unwrap();
                let mut t = Text::new("tester", &i_texture);
                t.x = x * (viewport.window_size.0 as i32 / d);
                t.y = y * (viewport.window_size.1 as i32 / d);
                e.add_component(t);
            }
        }
    }

    {
        let audio_data = app.resource.load_audio_data("res/snd/beep.wav").unwrap();
        let beep = AudioClip::new("beep", audio_data);
        e.add_component(beep);
    }

    {
        let mut button = make_button(app, viewport);
        button.name = "test_button".to_string();
        e.add_child(button);
    }

    {
        app.resource.load_image_from_disk_async(DISK_IMAGE_QUAD);

        let mut image = Image::new("test-quad");
        image.x = viewport.window_size.0 / 2.;
        image.y = viewport.window_size.1 / 2.;
        image.r_rect = Some(ImageRenderRect {
            x: 256.,
            y: 256.,
            w: 256.,
            h: 256.,
        });
        e.add_component(image);
    }

    {
        let mut animated_image = make_animated_image(
            app,
            "test-animated",
            DISK_IMAGE_MARIO,
            35.,
            50.,
            Some((3., 3.)),
            Some(10.),
        );
        animated_image.x = 500.;
        animated_image.y = 500.;
        animated_image_add_animation(&mut animated_image, "idle", (0, 0));
        animated_image_add_animation(&mut animated_image, "walking", (1, 4));
        animated_image_add_animation(&mut animated_image, "swim", (26, 31));
        animated_image_set_animation(&mut animated_image, "idle");

        e.add_child(animated_image);
    }

    e
}
