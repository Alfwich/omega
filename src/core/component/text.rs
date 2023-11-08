use crate::app::App;
use crate::core::component::Component;
use crate::core::renderer::MVPConfig;
use crate::core::resource::TextLoadInfo;

use crate::util::alpha::Alpha;
use crate::util::color::Color;
use crate::util::rect::Rect;
use crate::util::scale::Scale;

use core::ffi::c_void;

use gl::*;
extern crate nalgebra_glm as glm;

use core::any::Any;

#[derive(Default, Debug)]
pub struct Text {
    pub name: String,
    pub zindex: i32,
    pub text: String,
    pub texture_id: Option<u32>,
    pub x: i32,
    pub y: i32,
    pub rotation: f32,
    pub width: u32,
    pub height: u32,
    pub color: Color,
    pub alpha: Alpha,
}

impl Text {
    pub fn new(name: &str) -> Self {
        Text {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn new_with_text(name: &str, app: &mut App, t: &str) -> Self {
        let mut text = Text::new(name);
        text.update_text(
            app,
            &TextLoadInfo {
                text: t.to_string(),
                ..Default::default()
            },
        );

        text
    }

    pub fn update_text(&mut self, app: &mut App, text_load_info: &TextLoadInfo) {
        let text_texture = app.resource.load_text_texture(text_load_info).unwrap();
        self.texture_id = Some(text_texture.texture_id);
        self.width = text_texture.width;
        self.height = text_texture.height;
    }
}

impl Component for Text {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn z_index(&self) -> i32 {
        self.zindex
    }

    fn render(&self, app: &App, parent_offset: (f32, f32)) {
        if let Some(tid) = self.texture_id {
            let mvp = app.renderer.make_mvp(&MVPConfig {
                rect: Rect {
                    x: self.x as f32 + parent_offset.0,
                    y: self.y as f32 + parent_offset.1,
                    w: self.width as f32,
                    h: self.height as f32,
                },
                rotation: self.rotation,
                scale: Scale { x: 1., y: 1. },
            });

            unsafe {
                Enable(BLEND);
                BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
                BindVertexArray(app.renderer.gl.vao);
                BindBuffer(ELEMENT_ARRAY_BUFFER, app.renderer.gl.ebo);
                UseProgram(app.renderer.gl.text_program.id);
                UniformMatrix4fv(
                    app.renderer.gl.text_program.mvp_loc,
                    1,
                    FALSE,
                    mvp.data.as_slice().as_ptr(),
                );
                Uniform4f(
                    app.renderer.gl.text_program.color_loc,
                    self.color.r,
                    self.color.g,
                    self.color.b,
                    self.alpha.val,
                );
                BindTexture(TEXTURE_2D, tid);
                DrawElements(TRIANGLES, 6, UNSIGNED_INT, std::ptr::null::<c_void>());
            }
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
