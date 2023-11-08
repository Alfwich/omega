use crate::app::App;
use crate::core::component::Component;

use gl::*;

use core::any::Any;

#[derive(Default, Debug)]
pub struct PreFrame {}

impl Component for PreFrame {
    fn get_name(&self) -> &str {
        "__pre_frame__"
    }

    fn render(&self, app: &App, _parent_offset: (f32, f32)) {
        unsafe {
            Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            Viewport(
                0,
                0,
                app.renderer.viewport.window_size.0 as i32,
                app.renderer.viewport.window_size.1 as i32,
            );
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
