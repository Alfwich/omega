use crate::core::component::component::Component;

use core::any::Any;

#[derive(Default, Debug)]
pub struct Offset {
    pub x: f32,
    pub y: f32,
}

pub static OFFSET_NAME: &str = "__offset_name__";

impl Component for Offset {
    fn get_name(&self) -> &str {
        OFFSET_NAME
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
