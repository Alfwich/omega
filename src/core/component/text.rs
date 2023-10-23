use crate::core::component::component::Component;
use crate::core::renderer::renderer::Renderer;

use core::ffi::c_void;

use gl::*;
extern crate nalgebra_glm as glm;

use crate::core::renderer::app_gl::*;

use core::any::Any;

#[derive(Default, Debug)]
pub struct Text {
    pub name: String,
    pub text: String,
    pub texture_id: u32,
    pub x: i32,
    pub y: i32,
    pub rotation: f32,
    pub width: u32,
    pub height: u32,
}

impl Text {
    pub fn new(name: &str, text_texture: &Texture) -> Self {
        Text {
            name: name.to_string(),
            texture_id: text_texture.texture_id,
            width: text_texture.width,
            height: text_texture.height,
            ..Default::default()
        }
    }
}

impl Component for Text {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn render(&self, renderer: &Renderer) {
        let mvp = renderer.make_mvp(
            self.x as f32,
            self.y as f32,
            self.width as f32,
            self.height as f32,
            self.rotation,
            1.,
            1.,
        );

        unsafe {
            Enable(BLEND);
            BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
            BindVertexArray(renderer.gl.vao);
            BindBuffer(ELEMENT_ARRAY_BUFFER, renderer.gl.ebo);
            UseProgram(renderer.gl.text_program_id);
            UniformMatrix4fv(
                renderer.gl.text_program_mvp_loc,
                1,
                FALSE,
                mvp.data.as_slice().as_ptr(),
            );
            BindTexture(TEXTURE_2D, self.texture_id);
            DrawElements(TRIANGLES, 6, UNSIGNED_INT, 0 as *const c_void);
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
