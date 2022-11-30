use sfml::window::{Event, Key, Style, Window};

extern crate nalgebra_glm as glm;
extern crate sfml;

mod core;
mod game;
mod util;

use crate::core::audio::sound::Sound;
use crate::core::component::pre_frame::PreFrame;
use crate::core::entity::Entity;
use crate::game::state::GameState;
use crate::util::timer::Timer;

pub struct App {
    pub root: Entity,
    pub state: GameState,
}

impl Default for App {
    fn default() -> Self {
        App {
            root: Entity::new_noop("root"),
            state: GameState::default(),
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

fn main() {
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
        let mut beep = Sound::new("beep", "res/snd/beep.wav");

        app.root.components.push(Box::new(PreFrame::default()));
        app.root
            .children
            .push(crate::game::entities::title::make_title(&renderer.viewport));

        while window.is_open() {
            let dt = frame_timer.dt();

            if beep.get_sound().status() == sfml::audio::SoundStatus::STOPPED {
                beep.get_sound().play();
            }

            handle_window_events(&mut window, &mut app);

            update(&mut app, dt);

            window.set_active(true);

            app.root.render(&renderer);

            window.display();
        }
    }
}
