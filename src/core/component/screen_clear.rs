use crate::core::component::component::Component;
use crate::core::renderer::renderer::Renderer;

use gl::*;

use core::any::Any;

#[derive(Default, Debug)]
pub struct ScreenClear {
    pub name: String,
}

impl ScreenClear {
    pub fn new(name: &str) -> Self {
        ScreenClear {
            name: name.to_string(),
        }
    }
}

impl Component for ScreenClear {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn render(&self, renderer: &Renderer) {
        unsafe {
            Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            Enable(BLEND);
            BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
            Viewport(
                0,
                0,
                renderer.viewport.window_size.0 as i32,
                renderer.viewport.window_size.1 as i32,
            );
            BindVertexArray(renderer.gl.vao);
            BindBuffer(ELEMENT_ARRAY_BUFFER, renderer.gl.ebo);
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
