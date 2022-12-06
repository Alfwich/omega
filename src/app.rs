use crate::core::component::pre_frame::PreFrame;
use crate::core::entity::Entity;
use crate::core::event::Event::{ImageLoadEvent, SFMLEvent};
use crate::core::event::ImageLoadEventPayload;
use crate::core::renderer::renderer::Renderer;
use crate::core::resource::Resources;
use crate::game::entities::title;
use crate::game::state::GameState;
use crate::util::timer::Timer;
use sfml::system::Vector2i;
use sfml::window::{Event as SEvent, Key, Style, VideoMode, Window};

pub struct App {
    pub state: GameState,
    pub resource: Resources,
}

impl Default for App {
    fn default() -> Self {
        App {
            state: GameState::default(),
            resource: Resources::default(),
        }
    }
}

fn handle_window_events(window: &mut Window, app: &mut App, root: &mut Entity) {
    while let Some(event) = window.poll_event() {
        match event {
            SEvent::Closed => {
                window.close();
            }
            SEvent::KeyPressed { code, .. } => match code {
                Key::Q => {
                    window.close();
                }
                _ => {}
            },
            _ => {}
        }

        root.handle_event(app, &SFMLEvent(event));
    }

    let mut image_load_events = Vec::new();
    loop {
        if let Some(image_load_result) = app.resource.recv_load_events() {
            let load_info = image_load_result.1;
            image_load_events.push(ImageLoadEvent(ImageLoadEventPayload(
                image_load_result.0,
                load_info.texture_id,
                load_info.width,
                load_info.height,
            )));
        } else {
            break;
        }
    }

    for e in image_load_events.iter() {
        root.handle_event(app, e)
    }
}

pub fn run() {
    let dvm = VideoMode::desktop_mode();
    let window_size: (u32, u32) = (dvm.width, dvm.height);

    // Creates GL context internally
    let mut window = Window::new(window_size, "Omega", Style::NONE, &Default::default());
    window.set_position(Vector2i::new(0, 0));
    window.set_framerate_limit(0);
    window.set_vertical_sync_enabled(false);

    let mut renderer = Renderer::new(window_size.0 as f32, window_size.1 as f32);
    let mut frame_timer = Timer::default();
    let mut app = App::default();

    {
        let mut root = Entity::default();
        root.add_component(PreFrame::default());
        root.add_child(title::make_title(&mut app, &renderer.viewport));

        while window.is_open() {
            let dt = frame_timer.dt();

            //println!("fps: {}", 1. / dt);
            handle_window_events(&mut window, &mut app, &mut root);
            root.update(&app, dt);
            window.set_active(true);
            root.render(&mut renderer);
            window.display();
        }
    }
}
