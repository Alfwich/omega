use crate::core::renderer::renderer::Renderer;

pub trait Component {
    fn attached(&self);
    fn detached(&self);
    fn update(&mut self, dt: f32);
    fn render(&self, renderer: &Renderer);
}
