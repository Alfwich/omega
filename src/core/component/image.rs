use crate::core::component::component::Component;
use crate::core::renderer::renderer::Renderer;

use core::ffi::c_void;

use gl::*;
use glm::TVec3;
extern crate nalgebra_glm as glm;

use core::any::Any;

#[derive(Debug)]
pub struct ImageRenderRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Default for ImageRenderRect {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 1.0,
            h: 1.0,
        }
    }
}

#[derive(Default, Debug)]
pub struct Image {
    pub name: String,
    pub scale: f32,
    pub border: f32,
    pub texture_id: Option<u32>,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub width: f32,
    pub height: f32,
    pub color: TVec3<f32>,

    // Optional Section of the image to render
    pub r_rect: Option<ImageRenderRect>,
}

impl Image {
    pub fn new_nameless() -> Self {
        Self::new("")
    }

    pub fn new(name: &str) -> Self {
        Image {
            name: name.to_string(),
            scale: 1.,
            color: glm::make_vec3(&[1., 1., 1.]),
            ..Default::default()
        }
    }

    pub fn with_texture(name: &str, texture_id: u32, width: f32, height: f32) -> Self {
        Image {
            name: name.to_string(),
            texture_id: Some(texture_id),
            width,
            height,
            scale: 1.,
            color: glm::make_vec3(&[1., 1., 1.]),
            ..Default::default()
        }
    }
}

impl Component for Image {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn render(&self, renderer: &Renderer) {
        if let Some(texture_id) = self.texture_id {
            let mvp = renderer.make_mvp(
                self.x as f32,
                self.y as f32,
                self.width as f32,
                self.height as f32,
                self.rotation,
                self.scale,
                self.scale,
            );

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

                Uniform4f(
                    renderer.gl.image_program_color_loc,
                    self.color.x,
                    self.color.y,
                    self.color.z,
                    1.0,
                );

                match &self.r_rect {
                    Some(r) => {
                        Uniform4f(renderer.gl.image_program_uv_rect_loc, r.x, r.y, r.w, r.h);
                    }
                    _ => {
                        // Default rect which renders the whole image
                        Uniform4f(renderer.gl.image_program_uv_rect_loc, 0.0, 0.0, 1.0, 1.0);
                    }
                };

                BindTexture(TEXTURE_2D, texture_id);
                DrawElements(TRIANGLES, 6, UNSIGNED_INT, 0 as *const c_void);
            }
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
