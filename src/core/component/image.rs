use crate::core::component::component::Component;
use crate::core::renderer::renderer::Renderer;

use core::ffi::c_void;

use gl::*;
extern crate nalgebra_glm as glm;

use core::any::Any;

#[derive(Default, Debug)]
pub struct Image {
    pub name: String,
    pub scale: f32,
    pub border: f32,
    pub texture_id: u32,
    pub x: i32,
    pub y: i32,
    pub rotation: f32,
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn new(name: &str, texture_id: u32, width: u32, height: u32) -> Self {
        Image {
            name: name.to_string(),
            texture_id,
            width,
            height,
            ..Default::default()
        }
    }

    // TODO: Other methods which encapsalate ctor behavior
}

impl Component for Image {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn render(&self, renderer: &Renderer) {
        let scale = glm::make_vec3(&[self.width as f32, self.height as f32, 1.]);
        let scale_model = glm::scale(&renderer.id, &scale);
        let rotate_vec = glm::make_vec3(&[0., 0., 1.]);
        let rotate_model = glm::rotate(&renderer.id, self.rotation, &rotate_vec);
        let mve = glm::make_vec3(&[
            self.x as f32,
            renderer.viewport.window_size.1 - self.y as f32,
            0.,
        ]);
        let view = glm::translate(&renderer.id, &mve);
        let model = rotate_model * scale_model;
        let mvp = renderer.ortho * view * model;

        unsafe {
            Enable(BLEND);
            BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
            BindVertexArray(renderer.gl.vao);
            BindBuffer(ELEMENT_ARRAY_BUFFER, renderer.gl.ebo);
            UseProgram(renderer.gl.image_program_id);
            UniformMatrix4fv(
                renderer.gl.image_program_mvp_loc,
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
