use std::any::Any;

use crate::app::App;

pub trait Component {
    fn get_name(&self) -> &str;
    fn render(&self, _app: &App, _parent_offset: (f32, f32)) {}
    fn as_any(&mut self) -> &mut dyn Any;
}
