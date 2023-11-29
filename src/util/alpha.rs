#[derive(Debug)]
pub struct Alpha {
    pub val: f32,
}

impl Default for Alpha {
    fn default() -> Self {
        Self { val: 1. }
    }
}

impl Alpha {
    pub fn new(val: f32) -> Self {
        Self { val }
    }
}
