use sfml::window::{Event, Key, Style, Window};
use gl::*;

extern crate nalgebra_glm as glm;
extern crate sfml;

use crate::core::renderer::*;

mod core;
mod game;
mod util;

use crate::core::entity::Entity;
use crate::game::state::GameState;

use std::convert::TryInto;

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

fn update(_app: &mut App, _dt: f32) {}

fn pre_render(renderer: &crate::renderer::Renderer) {
    unsafe {
        Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        Enable(BLEND);
        BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
        Viewport(
            0,
            0,
            renderer.viewport.window_size.0 as i32,
            renderer.viewport.window_size.1 as i32,
        );
        BindVertexArray(renderer.gl.vao);
        BindBuffer(ELEMENT_ARRAY_BUFFER, renderer.gl.ebo);
    }
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

    let renderer =
        crate::core::renderer::renderer::Renderer::new(WINDOW_SIZE.0 as f32, WINDOW_SIZE.1 as f32);
    let mut app = App::default();
    let mut frame_timer = util::Timer::default();
    
    app.root.children.push(crate::game::entities::title::make_title());

    while window.is_open() {
        let dt = frame_timer.dt();

        handle_window_events(&mut window);

        update(&mut app, dt);
        app.root.update(dt);

        window.set_active(true);

        pre_render(&renderer);
        app.root.render(&renderer);

        window.display();
    }
}
