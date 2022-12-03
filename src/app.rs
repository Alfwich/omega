use sfml::window::{Event, Key, Style, Window};

use crate::core::component::pre_frame::PreFrame;
use crate::core::entity::Entity;
use crate::core::resource::Resources;
use crate::game::state::GameState;
use crate::util::timer::Timer;

pub struct App {
    pub root: Entity,
    pub state: GameState,
    pub resource: Resources,
}

impl Default for App {
    fn default() -> Self {
        App {
            root: Entity::default(),
            state: GameState::default(),
            resource: Resources::default(),
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
    app.resource.tick_loads();
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

            //println!("fps: {}", 1. / dt);
            handle_window_events(&mut window, &mut app);
            update(&mut app, dt);
            window.set_active(true);
            app.root.render(&renderer);
            window.display();
        }
    }
}
