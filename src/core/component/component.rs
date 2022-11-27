use crate::core::renderer::renderer::Renderer;

use std::any::Any;

pub trait Component {
    fn attached(&self);
    fn detached(&self);
    fn update(&mut self, dt: f32);
    fn render(&self, renderer: &Renderer);
    fn as_any(&mut self) -> &mut dyn Any;
}
