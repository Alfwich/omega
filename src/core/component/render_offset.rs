use crate::core::component::component::Component;
use crate::core::renderer::renderer::Renderer;

use core::any::Any;

#[derive(Default, Debug, Clone, Copy)]
pub struct RenderOffset {
    pub x: f32,
    pub y: f32,
}

impl Component for RenderOffset {
    fn get_name(&self) -> &str {
        "render_offset"
    }

    fn render(&self, _renderer: &Renderer) {}

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
