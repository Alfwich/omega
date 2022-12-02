use std::cell::RefCell;
use std::collections::HashMap;

use sfml::audio::SoundBuffer;
use sfml::window::{Event, Key, Style, Window};
use sfml::SfBox;

use crate::core::component::pre_frame::PreFrame;
use crate::core::entity::Entity;
use crate::game::state::GameState;
use crate::util::timer::Timer;

pub struct App {
    pub root: Entity,
    pub state: GameState,
    pub audio_data: HashMap<String, RefCell<SfBox<SoundBuffer>>>,
}

impl Default for App {
    fn default() -> Self {
        App {
            root: Entity::new_noop("root"),
            state: GameState::default(),
            audio_data: HashMap::new(),
        }
    }
}

impl App {
    pub fn load_audio_data(&mut self, audio_file_path: &str) {
        if (!self.audio_data.contains_key(audio_file_path)) {
            self.audio_data.insert(
                audio_file_path.to_string(),
                RefCell::new(sfml::audio::SoundBuffer::from_file(audio_file_path).unwrap()),
            );
        }
    }
}

fn handle_window_events(window: &mut Window, app: &mut App) {
    while let Some(event) = window.poll_event() {
        match event {
            Event::Closed => {
                window.close();
            }
            Event::KeyPressed { code, .. } => match code {
                Key::Q => {
                    window.close();
                }
                _ => {}
            },
            _ => {}
        }

        app.root.handle_event(&event);
    }
}

fn update(app: &mut App, dt: f32) {
    app.root.update(dt);
}

pub fn run() {
    static WINDOW_SIZE: (u32, u32) = (1920, 1080);

    // Creates GL context internally
    let mut window = Window::new(WINDOW_SIZE, "Omega", Style::CLOSE, &Default::default());
    window.set_framerate_limit(0);
    window.set_vertical_sync_enabled(false);

    let renderer =
        crate::core::renderer::renderer::Renderer::new(WINDOW_SIZE.0 as f32, WINDOW_SIZE.1 as f32);
    let mut frame_timer = Timer::default();
    let mut app = App::default();

    {
        app.root.components.push(Box::new(PreFrame::default()));
        let title = crate::game::entities::title::make_title(&mut app, &renderer.viewport);
        app.root.children.push(title);

        while window.is_open() {
            let dt = frame_timer.dt();

            handle_window_events(&mut window, &mut app);

            update(&mut app, dt);

            window.set_active(true);

            app.root.render(&renderer);

            window.display();
        }
    }
}
