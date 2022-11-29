use crate::core::renderer::renderer::Renderer;

use std::any::Any;

pub trait Component {
    fn get_name(&self) -> &str;
    fn render(&self, renderer: &Renderer);
    fn as_any(&mut self) -> &mut dyn Any;
}
