use sfml::window::{Event, Key, Style, Window};

extern crate nalgebra_glm as glm;
extern crate sfml;

mod core;
mod game;
mod util;

use crate::core::component::screen_clear::ScreenClear;
use crate::core::entity::Entity;
use crate::game::state::GameState;

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
    static WINDOW_FPS: u32 = 1000;

    // Creates GL context internally
    let mut window = Window::new(
        WINDOW_SIZE,
        "Omega Survival",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(WINDOW_FPS);

    let renderer =
        crate::core::renderer::renderer::Renderer::new(WINDOW_SIZE.0 as f32, WINDOW_SIZE.1 as f32);
    let mut app = App::default();
    let mut frame_timer = util::Timer::default();

    app.root.components.push(Box::new(ScreenClear::new("cls")));
    app.root
        .children
        .push(crate::game::entities::title::make_title(&renderer.viewport));

    while window.is_open() {
        let dt = frame_timer.dt();

        handle_window_events(&mut window, &mut app);

        update(&mut app, dt);

        window.set_active(true);

        app.root.render(&renderer);

        window.display();
    }
}
