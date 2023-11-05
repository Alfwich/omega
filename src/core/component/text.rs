use crate::app::App;
use crate::core::component::component::Component;
use crate::core::resource::TextLoadInfo;

use core::ffi::c_void;

use gl::*;
extern crate nalgebra_glm as glm;

use core::any::Any;

#[derive(Default, Debug)]
pub struct Text {
    pub name: String,
    pub text: String,
    pub texture_id: Option<u32>,
    pub x: i32,
    pub y: i32,
    pub rotation: f32,
    pub width: u32,
    pub height: u32,
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

    fn render(&self, app: &App, parent_offset: (f32, f32)) {
        if let Some(tid) = self.texture_id {
            let mvp = app.renderer.as_ref().unwrap().make_mvp(
                self.x as f32 + parent_offset.0,
                self.y as f32 + parent_offset.1,
                self.width as f32,
                self.height as f32,
                self.rotation,
                1.,
                1.,
            );

            unsafe {
                Enable(BLEND);
                BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
                BindVertexArray(app.renderer.as_ref().unwrap().gl.vao);
                BindBuffer(ELEMENT_ARRAY_BUFFER, app.renderer.as_ref().unwrap().gl.ebo);
                UseProgram(app.renderer.as_ref().unwrap().gl.text_program_id);
                UniformMatrix4fv(
                    app.renderer.as_ref().unwrap().gl.text_program_mvp_loc,
                    1,
                    FALSE,
                    mvp.data.as_slice().as_ptr(),
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
