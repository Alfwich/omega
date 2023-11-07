#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn update(&mut self, r: f32, g: f32, b: f32) {
        self.r = r;
        self.g = g;
        self.b = b;
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 1.,
            g: 1.,
            b: 1.,
        }
    }
}
