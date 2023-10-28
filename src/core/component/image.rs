use crate::core::component::component::Component;
use crate::core::renderer::app_gl::Texture;
use crate::core::renderer::renderer::Renderer;

use core::ffi::c_void;

use gl::*;
use glm::TVec3;
extern crate nalgebra_glm as glm;

use core::any::Any;



#[derive(Debug, Clone)]
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
    pub texture: Option<Texture>,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub width: f32,
    pub height: f32,
    pub color: TVec3<f32>,

    // Optional Section of the image to render in screen space
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

    pub fn with_texture(name: &str, texture: &Texture, width: f32, height: f32) -> Self {
        Image {
            name: name.to_string(),
            texture: Some(texture.clone()),
            width,
            height,
            scale: 1.,
            color: glm::make_vec3(&[1., 1., 1.]),
            ..Default::default()
        }
    }

    pub fn apply_image(
        &mut self,
        image_load_event_payload: &crate::core::event::ImageLoadEventPayload,
    ) {
        self.texture = Some(Texture {
            texture_id: image_load_event_payload.texture_id,
            width: image_load_event_payload.width,
            height: image_load_event_payload.height,
        });
        self.width = image_load_event_payload.width as f32;
        self.height = image_load_event_payload.height as f32;
    }
}

impl Component for Image {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn render(&self, renderer: &Renderer) {
        if let Some(texture) = self.texture {
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
                        // Convert screen space rect to render space
                        // 256, 256 ,256 ,256 => (0.5, 0.5, 0.5, 0.5) @ 512x512
                        let x = r.x / texture.width as f32;
                        let y = r.y / texture.height as f32;
                        let w = r.w / texture.width as f32;
                        let h = r.h / texture.height as f32;
                        Uniform4f(renderer.gl.image_program_uv_rect_loc, x, y, w, h);
                    }
                    _ => {
                        // Default rect which renders the whole image
                        Uniform4f(renderer.gl.image_program_uv_rect_loc, 0.0, 0.0, 1.0, 1.0);
                    }
                };

                BindTexture(TEXTURE_2D, texture.texture_id);
                DrawElements(TRIANGLES, 6, UNSIGNED_INT, 0 as *const c_void);
            }
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
