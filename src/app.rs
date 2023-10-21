use crate::core::component::pre_frame::PreFrame;
use crate::core::entity::Entity;
use crate::core::event::Event::{ImageLoadEvent, SFMLEvent};
use crate::core::event::ImageLoadEventPayload;
use crate::core::renderer::renderer::Renderer;
use crate::core::renderer::window::{make_window, WindowConfig, WindowStyle};
use crate::core::resource::Resources;
use crate::game::scene::title;
use crate::game::state::GameState;
use crate::util::timer::Timer;
use sfml::window::{Event as SEvent, Key, Window};

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
            image_load_events.push(ImageLoadEvent(ImageLoadEventPayload {
                url: image_load_result.0,
                texture_id: load_info.texture_id,
                width: load_info.width,
                height: load_info.height,
            }));
        } else {
            break;
        }
    }

    for e in image_load_events.iter() {
        root.handle_event(app, e)
    }
}

impl App {
    pub fn run(&mut self) {
        let mut window_config = WindowConfig::default();
        window_config.width = 1920;
        window_config.height = 1080;
        window_config.style = WindowStyle::Windowed;
        window_config.vsync_enabled = false;
        let mut window = make_window(&window_config);

        let mut renderer = Renderer::new(window_config.width as f32, window_config.height as f32);
        let mut frame_timer = Timer::default();

        {
            let mut root = Entity::default();
            root.add_component(PreFrame::default());
            root.add_child(title::make_title(self, &renderer.viewport));

            while window.is_open() {
                let dt = frame_timer.dt();

                //println!("fps: {}", 1. / dt);
                handle_window_events(&mut window, self, &mut root);
                root.update(self, dt);
                window.set_active(true);
                root.render_components(&mut renderer);
                window.display();
            }
        }
    }
}
