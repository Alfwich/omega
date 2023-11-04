use crate::app::App;
use crate::core::component::component::Component;

use gl::*;

use core::any::Any;

#[derive(Default, Debug)]
pub struct PreFrame {}

impl Component for PreFrame {
    fn get_name(&self) -> &str {
        "__pre_frame__"
    }

    fn render(&self, app: &App) {
        unsafe {
            Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            Viewport(
                0,
                0,
                app.renderer.as_ref().unwrap().viewport.window_size.0 as i32,
                app.renderer.as_ref().unwrap().viewport.window_size.1 as i32,
            );
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
