use sfml::window::*;

pub fn make_window(config: &WindowConfig) -> Box<Window> {
    // Creates GL context internally
    let sfml_window_style = match config.style {
        WindowStyle::Windowed => Style::CLOSE,
        WindowStyle::Fullscreen => Style::FULLSCREEN,
        WindowStyle::FullscreenBorderless => Style::NONE,
    };
    let mut result = Window::new(
        (config.width, config.height),
        "Omega",
        sfml_window_style,
        &Default::default(),
    );

    configure_sfml_window(&mut result, config);

    return Box::new(result);
}

#[allow(dead_code)]
#[derive(Default)]
pub enum WindowStyle {
    Windowed,
    Fullscreen,
    #[default]
    FullscreenBorderless,
}

#[derive(Default)]
pub struct WindowConfig {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) fps_limit: u32,
    pub(crate) vsync_enabled: bool,
    pub(crate) style: WindowStyle,
}

fn get_desktop_display_size() -> (u32, u32) {
    let dvm = VideoMode::desktop_mode();

    (dvm.width, dvm.height)
}

pub fn configure_sfml_window(w: &mut Window, config: &WindowConfig) {
    let desktop_size = get_desktop_display_size();
    if config.width == 0 || config.height == 0 {
        w.set_size(sfml::system::Vector2u::new(desktop_size.0, desktop_size.1));
    } else {
        w.set_size(sfml::system::Vector2u::new(config.width, config.height));
    }
    //window.set_position(Vector2i::new(0, 0));
    w.set_framerate_limit(config.fps_limit);
    w.set_vertical_sync_enabled(config.vsync_enabled);
}
