use crate::app::App;
use crate::core::component::audio_clip::AudioClip;
use crate::core::component::image::Image;
use crate::core::component::offset::{Offset, OFFSET_NAME};
use crate::core::component::text::Text;
use crate::core::component::Component;
use crate::core::entity::animated_image::{
    animated_image_add_animation, animated_image_get_position, animated_image_set_animation,
    make_animated_image,
};
use crate::core::entity::{Entity, EntityFns, RenderableEntity};
use crate::core::event::Event;

use crate::core::resource::{AsyncLoadHandle, TextLoadInfo};
use crate::game::entity::button::make_button;
use crate::util::alpha::Alpha;

use rand::Rng;
use sfml::window::{Event as SFMLEvent, Key};

use core::any::Any;
use std::f32::consts::PI;

#[derive(Default, Debug, Clone, Copy)]
struct Data {
    counter: f32,
    left_down: bool,
    right_down: bool,
    parent_offset: (f32, f32),
    time_scale: f32,

    sync_loaded_texture_id: u32,
    async_local_handle: Option<AsyncLoadHandle>,
    async_remote_handle: Option<AsyncLoadHandle>,
}

impl Component for Data {
    fn get_name(&self) -> &str {
        "data"
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

static REMOTE_IMAGE_URL: &str = "http://wuteri.ch/img/Teleport.jpg";
static DISK_IMAGE_PATH: &str = "res/img/motorcycle.png";
static DISK_IMAGE_QUAD: &str = "res/img/test-clip.png";
static DISK_IMAGE_MARIO: &str = "res/img/mario.png";

fn update_testbed(e: &mut Entity, app: &App, in_dt: f32) {
    //println!("fps: {}", 1. / dt);

    let d;
    let dt;
    {
        let data = e.find_component::<Data>("data").unwrap();
        dt = in_dt * data.time_scale;
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
        button.set_x(d.counter.cos() * 50. * PI * 2. + 300.);
        button.set_y(d.counter.sin() * 50. * PI * 2. + 300.);
    }

    {
        let offset = e.find_component::<Offset>(OFFSET_NAME).unwrap();
        offset.x = d.counter.cos() * 10. * PI * 2.;
        offset.y = d.counter.sin() * 10. * PI * 2.;
    }
    {
        let mario = e.find_child_by_name("test-animated").unwrap();
        if mario.active {
            match (d.left_down, d.right_down) {
                (true, false) => {
                    mario.move_x(-100. * dt);
                }
                (false, true) => {
                    mario.move_x(100. * dt);
                }
                _ => {}
            }
            let mario_location = animated_image_get_position(mario);
            if mario_location.1 < app.renderer.viewport.window_size.1 / 2. {
                mario.zindex = -1;
            } else {
                mario.zindex = 1;
            }
        }
    }
}

fn handle_event(e: &mut Entity, app: &mut Option<&mut App>, ev: &Event) {
    match ev {
        Event::SFMLEvent(ev) => match ev {
            SFMLEvent::Closed => {
                app.as_mut().unwrap().close_window();
            }
            SFMLEvent::MouseMoved { x, y } => {
                let offset = e.find_component::<Data>("data").unwrap().parent_offset;
                let card = e.find_component::<Image>("card").unwrap();
                card.x = *x as f32 - offset.0;
                card.y = *y as f32 - offset.1;
            }
            SFMLEvent::KeyPressed { code, .. } => match *code {
                Key::W => {
                    let animated_image = e.find_child_by_name("test-animated").unwrap();
                    if animated_image.active {
                        animated_image.move_y(-10.);
                    }
                }
                Key::A => {
                    {
                        let animated_image = e.find_child_by_name("test-animated").unwrap();
                        if animated_image.active {
                            animated_image_set_animation(animated_image, "walking");
                            animated_image.set_scale_x(-3.);
                        }
                    }
                    {
                        let data = e.find_component::<Data>("data").unwrap();
                        data.left_down = true;
                    }
                }
                Key::S => {
                    let animated_image = e.find_child_by_name("test-animated").unwrap();
                    if animated_image.active {
                        animated_image_set_animation(animated_image, "swim");
                        animated_image.move_y(10.);
                    }
                }
                Key::D => {
                    {
                        let animated_image = e.find_child_by_name("test-animated").unwrap();
                        if animated_image.active {
                            animated_image_set_animation(animated_image, "walking");
                            animated_image.set_scale_x(3.);
                        }
                    }
                    {
                        let data = e.find_component::<Data>("data").unwrap();
                        data.right_down = true;
                    }
                }
                Key::U => {
                    if let Some(a) = app {
                        let info = a.resource.load_image_from_disk(DISK_IMAGE_PATH).unwrap();
                        let mut dynamic_cmp = Image::default();
                        let mut thread_rng = rand::thread_rng();
                        dynamic_cmp.texture = Some(info);
                        dynamic_cmp.x = thread_rng.gen_range(0f32..1000f32);
                        dynamic_cmp.y = thread_rng.gen_range(0f32..1000f32);
                        dynamic_cmp.width = info.width as f32;
                        dynamic_cmp.height = info.height as f32;
                        dynamic_cmp.color.r = thread_rng.gen_range(0f32..1f32);
                        dynamic_cmp.color.g = thread_rng.gen_range(0f32..1f32);
                        dynamic_cmp.color.b = thread_rng.gen_range(0f32..1f32);
                        e.add_component(dynamic_cmp);
                    }
                }
                Key::Q => {
                    app.as_mut().unwrap().close_window();
                }

                Key::LBracket => {
                    let animated_image = e.find_child_by_name("test-animated").unwrap();
                    animated_image.active = false;
                    animated_image.set_color_mod(0.5, 0.5, 0.5);
                    animated_image.set_alpha(0.5);
                }

                Key::RBracket => {
                    let animated_image = e.find_child_by_name("test-animated").unwrap();
                    animated_image.active = true;
                    animated_image.set_color_mod(1., 1., 1.);
                    animated_image.set_alpha(1.);
                }

                Key::Num9 => {
                    let animated_image = e.find_child_by_name("test-animated").unwrap();
                    animated_image.visible = false;
                }

                Key::Num0 => {
                    let animated_image = e.find_child_by_name("test-animated").unwrap();
                    animated_image.visible = true;
                }

                Key::Num7 => {
                    let mut config = app.as_ref().unwrap().get_window_config();
                    config.width = 1280;
                    config.height = 720;
                    app.as_mut().unwrap().update_window_config(&config);
                }

                Key::Num8 => {
                    let mut config = app.as_ref().unwrap().get_window_config();
                    config.width = 1920;
                    config.height = 1080;
                    app.as_mut().unwrap().update_window_config(&config);
                }
                _ => {}
            },
            SFMLEvent::KeyReleased { code, .. } => match *code {
                Key::A => {
                    {
                        let animated_image = e.find_child_by_name("test-animated").unwrap();
                        if animated_image.active {
                            animated_image_set_animation(animated_image, "idle");
                        }
                    }

                    {
                        let data = e.find_component::<Data>("data").unwrap();
                        data.left_down = false;
                    }
                }
                Key::D => {
                    {
                        let animated_image = e.find_child_by_name("test-animated").unwrap();
                        if animated_image.active {
                            animated_image_set_animation(animated_image, "idle");
                        }
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
            let data = *e.find_component::<Data>("data").unwrap();
            let handle_id = img_data.handle.id;

            if let Some(async_handle) = data.async_remote_handle {
                if async_handle.id == handle_id {
                    let card = e.find_component::<Image>("card").unwrap();
                    card.apply_image(img_data);
                }
            }

            if let Some(async_handle) = data.async_local_handle {
                if async_handle.id == handle_id {
                    assert!(
                        img_data.texture_id == data.sync_loaded_texture_id,
                        "Expect async to return sync'd loaded texture"
                    );
                    let async_local = e.find_component::<Image>("async_local").unwrap();
                    async_local.apply_image(img_data)
                }
            }
        }
        Event::WindowUpdated(config) => {
            let quad = e.find_child_by_name("test-quad").unwrap();
            quad.set_x((config.width / 2) as f32);
            quad.set_y((config.height / 2) as f32);
        }
        _ => {}
    }
}

fn prerender_testbed(e: &mut Entity, parent_offset: (f32, f32)) {
    e.find_component::<Data>("data").unwrap().parent_offset = parent_offset;
}

pub fn make_testbed(app: &mut App, time_scale: f32) -> Entity {
    let mut e = Entity::new(
        "testbed",
        EntityFns {
            update_fn: update_testbed,
            event_fn: handle_event,
            prerender_fn: prerender_testbed,
        },
    );

    let mut data = Data {
        time_scale,
        ..Default::default()
    };

    e.add_component(Offset::default());

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
        image.zindex = -5;
        image.x = app.renderer.viewport.window_size.0 / 2.;
        image.y = app.renderer.viewport.window_size.1 / 2.;
        image.alpha = Alpha::new(0.2);
        e.add_component(image);
    }

    {
        data.async_local_handle = app
            .resource
            .load_image_from_disk_async(DISK_IMAGE_PATH)
            .ok();

        // sync load this to ensure corner case with async loading works as expected. The will immediatly
        // load DISK_IMAGE_PATH and upload this to the GPU. Once the async call gets resolved it should return
        // the same resource as this sync load and prevent leaking memory.
        let sync_load = app.resource.load_image_from_disk(DISK_IMAGE_PATH);
        let sync_load_2 = app.resource.load_image_from_disk(DISK_IMAGE_PATH);

        // This will be checked once we finish the async action
        data.sync_loaded_texture_id = sync_load.unwrap().texture_id;

        assert!(
            data.sync_loaded_texture_id == sync_load_2.unwrap().texture_id,
            "Expected that two sync loaded textures result in the same texture"
        );

        let mut async_local = Image::new("async_local");
        async_local.x = 1000.;
        async_local.y = 1000.;
        e.add_component(async_local);
    }

    {
        data.async_remote_handle = app
            .resource
            .load_image_from_url_async(REMOTE_IMAGE_URL)
            .ok();
        e.add_component(Image::new("card"));
    }

    {
        let mut image = make_animated_image(
            app,
            "test-quad",
            DISK_IMAGE_QUAD,
            256.,
            256.,
            Some(1.),
            Some(crate::core::component::image::ImageRenderType::Linear),
        );

        image.set_x(app.renderer.viewport.window_size.0 / 2.);
        image.set_y(app.renderer.viewport.window_size.1 / 2.);
        e.add_child(image);
    }

    {
        let mut text = Text::new_with_text("title", app, "Omega Ω");
        text.x = (app.renderer.viewport.window_size.0 / 2.) as i32;
        text.y = (app.renderer.viewport.window_size.1 / 2.) as i32;
        text.alpha.val = 0.5;
        text.color.r = 1.0;
        text.color.g = 0.;
        text.color.b = 0.;
        e.add_component(text);

        let d = 3;
        for x in 1..d {
            for y in 1..d {
                let text = format!("Omega {:?}:{:?}", x, y).to_string();
                let text_info = match (x + y) % 2 {
                    0 => TextLoadInfo {
                        text,
                        font_path: "res/font/Bicycle.otf".to_string(),
                        font_size: 24,
                    },
                    _ => TextLoadInfo {
                        text,
                        ..Default::default()
                    },
                };
                let mut t = Text::new("tester");
                t.update_text(app, &text_info);
                t.x = x * (app.renderer.viewport.window_size.0 as i32 / d);
                t.y = y * (app.renderer.viewport.window_size.1 as i32 / d);
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
        let mut button = make_button(app);
        button.name = "test_button".to_string();
        e.add_child(button);
    }

    {
        let mut animated_image = make_animated_image(
            app,
            "test-animated",
            DISK_IMAGE_MARIO,
            35.,
            50.,
            Some(10.),
            Some(crate::core::component::image::ImageRenderType::Nearest),
        );
        animated_image.set_x(500.);
        animated_image.set_y(500.);
        animated_image.set_scale_x(3.);
        animated_image.set_scale_y(3.);
        animated_image.set_width(35.);
        animated_image.set_height(50.);
        animated_image.set_rotation(45.);
        animated_image_add_animation(&mut animated_image, "idle", (0, 0));
        animated_image_add_animation(&mut animated_image, "walking", (1, 4));
        animated_image_add_animation(&mut animated_image, "swim", (26, 31));
        animated_image_set_animation(&mut animated_image, "idle");

        e.add_child(animated_image);
    }

    e.add_component(data);

    e
}
