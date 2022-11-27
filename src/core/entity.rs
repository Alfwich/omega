use crate::core::component::component::Component;

#[derive(Default)]
pub struct Entity {
    pub ctr: f32,
    pub components: Vec<Box<dyn Component>>,
}

impl Entity {
}
