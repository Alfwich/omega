use std::any::Any;

use crate::app::App;

pub trait Component {
    fn get_name(&self) -> &str;
    fn render(&self, _app: &App) {}
    fn as_any(&mut self) -> &mut dyn Any;
}
