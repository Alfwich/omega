use crate::core::component::component::Component;
use crate::core::renderer::renderer::Renderer;

use core::ffi::c_void;

use gl::*;
extern crate nalgebra_glm as glm;

use crate::core::renderer::app_gl;

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
    pub fn new(name: &str, text: &str) -> Self {
        let title_text = app_gl::render_text_to_texture(text).unwrap();
        Text {
            name: name.to_string(),
            text: text.to_string(),
            texture_id: title_text.texture_id,
            width: title_text.width,
            height: title_text.height,
            ..Default::default()
        }
    }
}

impl Drop for Text {
    fn drop(&mut self) {
        app_gl::release_texture(self.texture_id);
    }
}

impl Component for Text {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn render(&self, renderer: &Renderer) {
        let scale = glm::make_vec3(&[self.width as f32, self.height as f32, 1.]);
        let scale_model = glm::scale(&renderer.id, &scale);
        let rotate_vec = glm::make_vec3(&[0., 0., 1.]);
        let rotate_model = glm::rotate(&renderer.id, self.rotation, &rotate_vec);
        let mve = glm::make_vec3(&[
            renderer.viewport.window_size.0 as f32 / 2. + self.x as f32,
            renderer.viewport.window_size.1 as f32 / 2. + self.y as f32,
            0.,
        ]);
        let view = glm::translate(&renderer.id, &mve);
        let model = rotate_model * scale_model;
        let mvp = renderer.ortho * view * model;

        unsafe {
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
