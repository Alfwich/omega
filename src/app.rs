use sfml::system::Vector2;
use sfml::window::Window;

use crate::core::component::pre_frame::PreFrame;
use crate::core::entity::Entity;
use crate::core::event::Event::{self, ImageLoadEvent, SFMLEvent};
use crate::core::event::ImageLoadEventPayload;
use crate::core::renderer::window::{make_window, WindowConfig, WindowStyle};
use crate::core::renderer::Renderer;
use crate::core::resource::Resources;
use crate::game::scene::entry::make_entry;
use crate::game::state::GameState;
use crate::util::timer::Timer;

#[derive(Default)]
pub struct App {
    window: Option<Window>,
    window_config: WindowConfig,
    app_events: Vec<Event>,
    pub state: GameState,
    pub resource: Resources,
    pub renderer: Renderer,
}

impl App {
    pub fn close_window(&mut self) {
        if self.window.is_some() {
            self.window.as_mut().unwrap().close();
        }
    }

    pub fn update_window_config(&mut self, config: &WindowConfig) {
        if self.window.is_some() {
            self.window
                .as_mut()
                .unwrap()
                .set_size(Vector2::new(config.width, config.height));
            self.window
                .as_mut()
                .unwrap()
                .set_framerate_limit(config.fps_limit);
            self.window
                .as_mut()
                .unwrap()
                .set_vertical_sync_enabled(config.vsync_enabled);
            self.renderer
                .update_size(config.width as f32, config.height as f32);

            self.app_events.push(Event::WindowUpdated(config.clone()));
        }
    }

    pub fn get_window_config(&self) -> WindowConfig {
        self.window_config.clone()
    }

    fn handle_events(&mut self, root: &mut Entity) {
        // Handle any queued up application events first
        {
            let mut app_events = Vec::new();
            for e in &self.app_events {
                app_events.push(e.clone());
            }

            for e in app_events {
                root.handle_event(&mut Some(self), &e);
            }
        }

        // Handle async events
        {
            let mut image_load_events = Vec::new();
            while let Some(image_load_result) = self.resource.recv_load_events() {
                let load_info = image_load_result.1;
                image_load_events.push(ImageLoadEvent(ImageLoadEventPayload {
                    handle: load_info.handle,
                    texture_id: load_info.texture_id,
                    width: load_info.width,
                    height: load_info.height,
                }));
            }

            for e in image_load_events.iter() {
                root.handle_event(&mut Some(self), e)
            }
        }

        // Lastly, handle SFML window events
        {
            if self.window.is_some() {
                while let Some(event) = self.window.as_mut().unwrap().poll_event() {
                    root.handle_event(&mut Some(self), &SFMLEvent(event));
                }
            }
        }
    }

    pub fn run(&mut self) {
        self.window_config.title = "Omega".to_string();
        self.window_config.width = 1920;
        self.window_config.height = 1080;
        self.window_config.style = WindowStyle::Windowed;
        self.window_config.vsync_enabled = false;
        self.window = Some(*make_window(&self.window_config));

        // GL MUST be init after the window since this requires a valid GL context
        self.renderer.init_gl();
        self.renderer.update_size(
            self.window_config.width as f32,
            self.window_config.height as f32,
        );
        let mut frame_timer = Timer::default();

        {
            let mut root = Entity::default();
            root.add_component(PreFrame::default());
            root.add_child(make_entry(self));

            while self.window.as_ref().unwrap().is_open() {
                let dt = frame_timer.dt();

                self.handle_events(&mut root);
                root.update(self, dt);
                root.reorder_children();
                self.window.as_mut().unwrap().set_active(true);
                root.render_components(self, (0., 0.));
                self.window.as_mut().unwrap().display();
            }
        }
    }
}
