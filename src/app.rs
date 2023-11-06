use sfml::window::Window;

use crate::core::component::pre_frame::PreFrame;
use crate::core::entity::entity::Entity;
use crate::core::event::Event::{ImageLoadEvent, SFMLEvent};
use crate::core::event::ImageLoadEventPayload;
use crate::core::renderer::renderer::Renderer;
use crate::core::renderer::window::{make_window, WindowConfig, WindowStyle};
use crate::core::resource::Resources;
use crate::game::scene::testbed::make_testbed;
use crate::game::state::GameState;
use crate::util::timer::Timer;

#[derive(Default)]
pub struct App {
    pub window: Option<Window>,
    pub state: GameState,
    pub resource: Resources,
    pub renderer: Option<Renderer>,
}

impl App {
    pub fn close_window(&mut self) {
        if self.window.is_some() {
            self.window.as_mut().unwrap().close();
        }
    }

    fn handle_window_events(&mut self, root: &mut Entity) {
        if self.window.is_some() {
            while let Some(event) = self.window.as_mut().unwrap().poll_event() {
                /*
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
                */

                root.handle_event(&mut Some(self), &SFMLEvent(event));
            }
        }

        let mut image_load_events = Vec::new();
        loop {
            if let Some(image_load_result) = self.resource.recv_load_events() {
                let load_info = image_load_result.1;
                image_load_events.push(ImageLoadEvent(ImageLoadEventPayload {
                    handle: load_info.handle,
                    texture_id: load_info.texture_id,
                    width: load_info.width,
                    height: load_info.height,
                }));
            } else {
                break;
            }
        }

        for e in image_load_events.iter() {
            root.handle_event(&mut Some(self), e)
        }
    }

    pub fn run(&mut self) {
        let mut window_config = WindowConfig::default();
        window_config.width = 1920;
        window_config.height = 1080;
        window_config.style = WindowStyle::Windowed;
        window_config.vsync_enabled = false;
        self.window = Some(*make_window(&window_config));

        self.renderer = Some(Renderer::new(
            window_config.width as f32,
            window_config.height as f32,
        ));
        let mut frame_timer = Timer::default();

        {
            let mut root = Entity::default();
            root.add_component(PreFrame::default());
            root.add_child(make_testbed(self));

            while self.window.as_ref().unwrap().is_open() {
                let dt = frame_timer.dt();

                //println!("fps: {}", 1. / dt);
                self.handle_window_events(&mut root);
                root.update(self, dt);
                self.window.as_mut().unwrap().set_active(true);
                root.render_components(self, (0., 0.));
                self.window.as_mut().unwrap().display();
            }
        }
    }
}
