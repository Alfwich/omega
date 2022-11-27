use crate::core::component::component::Component;
use crate::core::renderer::renderer::Renderer;

use core::f32::consts::PI;
use core::ffi::c_void;

use gl::*;
extern crate nalgebra_glm as glm;

#[derive(Default, Debug)]
pub struct Image {
    pub scale: f32,
    pub border: f32,
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
}

impl Component for Image {
    
    fn attached(&self) {}
    
    fn detached(&self) {}
    
    fn update(&self, dt: f32) {
        println!("Hello World! {:?}", dt);
    }
    
    fn render(&self, renderer: &Renderer) {
            let scale = glm::make_vec3(&[
                self.width as f32,
                self.height as f32,
                1.,
            ]);
            
            let scale_model = glm::scale(&renderer.id, &scale);
            let rotate_vec = glm::make_vec3(&[0., 0., 1.]);
            let rotate_model = glm::rotate(&renderer.id, PI * 2.0 / 8.0, &rotate_vec);
            let mve = glm::make_vec3(&[
                (renderer.viewport.window_size.0 / 2.) / 2.,
                (renderer.viewport.window_size.1 / 2.) / 2. - 0.5,
                0.,
            ]);
            let view = glm::translate(&renderer.id, &mve);
            let model = rotate_model * scale_model;
            let mvp = renderer.ortho * view * model;

        unsafe {
            UseProgram(renderer.gl.tile_program_id);
            UniformMatrix4fv(
                renderer.gl.tile_program_mvp_loc,
                1,
                FALSE,
                mvp.data.as_slice().as_ptr(),
            );
            Uniform1f(renderer.gl.tile_program_border_loc, 0.);
            BindTexture(TEXTURE_2D, self.texture_id);
            DrawElements(TRIANGLES, 6, UNSIGNED_INT, 0 as *const c_void);
        }
    }
}