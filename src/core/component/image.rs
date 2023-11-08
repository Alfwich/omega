use crate::app::App;
use crate::core::component::Component;
use crate::core::renderer::app_gl::Texture;
use crate::core::renderer::MVPConfig;

use crate::util::alpha::Alpha;
use crate::util::color::Color;
use crate::util::rect::Rect;
use crate::util::scale::Scale;

use core::ffi::c_void;

use gl::*;

extern crate nalgebra_glm as glm;

use core::any::Any;

#[derive(Debug)]
pub enum ImageRenderType {
    Nearest,
    Linear,
}

#[derive(Debug, Default)]
pub struct Image {
    pub name: String,
    pub scale: Scale,
    pub zindex: i32,
    pub border: f32,
    pub texture: Option<Texture>,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub alpha: Alpha,

    // Optional Section of the image to render in screen space
    pub r_rect: Option<Rect>,

    pub render_type: Option<ImageRenderType>,
}

impl Image {
    pub fn new(name: &str) -> Self {
        Image {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn with_texture(name: &str, texture: &Texture, width: f32, height: f32) -> Self {
        Image {
            name: name.to_string(),
            texture: Some(*texture),
            width,
            height,
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

    fn z_index(&self) -> i32 {
        self.zindex
    }

    fn render(&self, app: &App, parent_offset: (f32, f32)) {
        if let Some(texture) = self.texture {
            let mvp = app.renderer.make_mvp(&MVPConfig {
                rect: Rect {
                    x: self.x + parent_offset.0,
                    y: self.y + parent_offset.1,
                    w: self.width,
                    h: self.height,
                },
                rotation: self.rotation,
                scale: self.scale,
            });

            unsafe {
                Enable(BLEND);
                BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
                BindVertexArray(app.renderer.gl.vao);
                BindBuffer(ELEMENT_ARRAY_BUFFER, app.renderer.gl.ebo);
                UseProgram(app.renderer.gl.image_program.id);

                UniformMatrix4fv(
                    app.renderer.gl.image_program.mvp_loc,
                    1,
                    FALSE,
                    mvp.data.as_slice().as_ptr(),
                );

                Uniform4f(
                    app.renderer.gl.image_program.color_loc,
                    self.color.r,
                    self.color.g,
                    self.color.b,
                    self.alpha.val,
                );

                match &self.r_rect {
                    Some(r) => {
                        // Convert screen space rect to render space
                        // 256, 256 ,256 ,256 => (0.5, 0.5, 0.5, 0.5) @ 512x512
                        let x = r.x / texture.width as f32;
                        let y = r.y / texture.height as f32;
                        let w = r.w / texture.width as f32;
                        let h = r.h / texture.height as f32;
                        Uniform4f(app.renderer.gl.image_program.uv_rect_loc, x, y, w, h);
                    }
                    _ => {
                        // Default rect which renders the whole image
                        Uniform4f(
                            app.renderer.gl.image_program.uv_rect_loc,
                            0.0,
                            0.0,
                            1.0,
                            1.0,
                        );
                    }
                };

                BindTexture(TEXTURE_2D, texture.texture_id);

                if let Some(t) = &self.render_type {
                    let min_max: (i32, i32) = match t {
                        ImageRenderType::Linear => (
                            NEAREST_MIPMAP_NEAREST.try_into().unwrap(),
                            NEAREST.try_into().unwrap(),
                        ),

                        _ => (
                            NEAREST_MIPMAP_NEAREST.try_into().unwrap(),
                            NEAREST.try_into().unwrap(),
                        ),
                    };
                    TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, min_max.0);
                    TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, min_max.1);
                }

                DrawElements(TRIANGLES, 6, UNSIGNED_INT, std::ptr::null::<c_void>());
            }
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
