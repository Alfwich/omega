use sfml::window::{Event, Key, Style, Window};

extern crate nalgebra_glm as glm;
extern crate sfml;

use crate::core::renderer::*;

mod core;
mod game;
mod util;

use crate::core::entity::Entity;
use crate::game::state::GameState;

#[derive(Default)]
pub struct App {
    pub root: Entity,
    pub state: GameState,
}

fn handle_window_events(window: &mut Window) {
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
    }
}

fn update(app: &mut App, dt: f32) {
    app.root.ctr += dt;
}

fn main() {
    static WINDOW_SIZE: (u32, u32) = (1920, 1080);
    static WINDOW_FPS: u32 = 200;

    // Creates GL context internally
    let mut window = Window::new(
        WINDOW_SIZE,
        "Omega Survival",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(WINDOW_FPS);

    let renderer = crate::core::renderer::renderer::Renderer::new(WINDOW_SIZE.0 as f32, WINDOW_SIZE.1 as f32);
    let mut app = App::default();
    let mut frame_timer = util::Timer::default();

    while window.is_open() {
        let dt = frame_timer.dt();

        handle_window_events(&mut window);

        update(&mut app, dt);

        window.set_active(true);

        unsafe {
            app_gl::render(&app, &renderer);
        }

        window.display();
    }
}
