pub mod app_gl;
pub mod window;

extern crate nalgebra_glm as glm;

use crate::{
    core::renderer::app_gl::AppGL,
    util::{rect::Rect, scale::Scale},
};

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
    pub offset: (f32, f32),
    pub gl: AppGL,
    pub id: glm::TMat4<f32>,
    pub ortho: glm::TMat4<f32>,
    pub viewport: Viewport,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            offset: (0., 0.),
            id: glm::identity::<f32, 4>(),
            ortho: glm::ortho(0.0f32, 1920., 0., 1080., -10., 100.),
            viewport: Viewport::new(1920., 1080.),
            gl: AppGL::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MVPConfig {
    pub rect: Rect,
    pub rotation: f32,
    pub scale: Scale,
}

impl Renderer {
    pub fn init_gl(&mut self) {
        self.gl.init();
    }

    pub fn update_size(&mut self, width: f32, height: f32) {
        self.ortho = glm::ortho(0.0f32, width, 0., height, -10., 100.);
        self.viewport = Viewport::new(width, height);
    }

    pub fn make_mvp(&self, cfg: &MVPConfig) -> glm::TMat4<f32> {
        let scale = glm::make_vec3(&[cfg.rect.w * cfg.scale.x, cfg.rect.h * cfg.scale.y, 1.]);
        let scale_model = glm::scale(&self.id, &scale);
        let rotate_vec = glm::make_vec3(&[0., 0., 1.]);
        let rotate_model = glm::rotate(&self.id, cfg.rotation, &rotate_vec);
        let mve = glm::make_vec3(&[
            cfg.rect.x + self.offset.0,
            self.viewport.window_size.1 - cfg.rect.y - self.offset.1,
            0.,
        ]);
        let view = glm::translate(&self.id, &mve);
        let model = rotate_model * scale_model;
        self.ortho * view * model
    }
}
