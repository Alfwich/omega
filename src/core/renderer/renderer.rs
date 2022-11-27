extern crate nalgebra_glm as glm;

use crate::core::renderer::app_gl::AppGL;

#[derive(Debug)]
pub struct Viewport {
    pub pos: [f32; 2],
    pub window_size: (f32, f32)
}

impl Viewport {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        Viewport { 
            pos: [0., 0.],
            window_size: (window_width, window_height)
        }
    }
}

pub struct Renderer {
    pub gl: AppGL,
    pub id: glm::TMat4<f32>,
    pub ortho: glm::TMat4<f32>,
    pub viewport: Viewport,
}

impl Renderer {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        Renderer {
            id: glm::identity::<f32, 4>(),
            ortho: glm::ortho(
                0.0f32,
                window_width,
                0.,
                window_height,
                -10.,
                100.,
            ),
            viewport: Viewport::new(window_width, window_height),
            gl: AppGL::default()
        }
    }
}
