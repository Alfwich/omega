extern crate nalgebra_glm as glm;

use crate::core::renderer::app_gl::AppGL;

#[derive(Debug)]
pub struct Viewport {
    pub offset: [f32; 2],
    pub window_size: (f32, f32),
}

impl Viewport {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        Viewport {
            offset: [0., 0.],
            window_size: (window_width, window_height),
        }
    }
}

pub struct Renderer {
    view_stack: Vec<(f32, f32)>,
    pub gl: AppGL,
    pub id: glm::TMat4<f32>,
    pub ortho: glm::TMat4<f32>,
    pub viewport: Viewport,
}

impl Renderer {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        Renderer {
            view_stack: Vec::new(),
            id: glm::identity::<f32, 4>(),
            ortho: glm::ortho(0.0f32, window_width, 0., window_height, -10., 100.),
            viewport: Viewport::new(window_width, window_height),
            gl: AppGL::default(),
        }
    }

    pub fn push_offset(&mut self, offset: (f32, f32)) {
        self.view_stack.push(offset);
    }

    pub fn pop_offset(&mut self) {
        self.view_stack.pop();
    }

    pub fn get_offset(&self) -> (f32, f32) {
        let mut result = (0., 0.);
        for offset in self.view_stack.iter() {
            result.0 += offset.0;
            result.1 += offset.1;
        }
        result
    }

    pub fn make_mvp(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        rotation: f32,
        scale_x: f32,
        scale_y: f32,
    ) -> glm::TMat4<f32> {
        let scale = glm::make_vec3(&[width * scale_x, height * scale_y, 1.]);
        let scale_model = glm::scale(&self.id, &scale);
        let rotate_vec = glm::make_vec3(&[0., 0., 1.]);
        let rotate_model = glm::rotate(&self.id, rotation, &rotate_vec);
        let offset = self.get_offset();
        let mve = glm::make_vec3(&[x + offset.0, self.viewport.window_size.1 - y - offset.1, 0.]);
        let view = glm::translate(&self.id, &mve);
        let model = rotate_model * scale_model;
        self.ortho * view * model
    }
}
