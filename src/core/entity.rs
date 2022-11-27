use crate::core::component::component::Component;
use crate::core::renderer::renderer::Renderer;

pub struct Entity {
    pub update_fn: fn(&mut Entity, dt: f32),
    pub components: Vec<Box<dyn Component>>,
    pub children: Vec<Entity>,
}

impl Entity {
    pub fn new(update: fn(&mut Entity, dt: f32)) -> Self {
        Entity {
            update_fn: update,
            components: Vec::default(),
            children: Vec::default(),
        }
    }
}

impl Entity {
    pub fn update(&mut self, dt: f32) {
        
        (self.update_fn)(self, dt);
        
        for cmp in &mut self.components {
            cmp.update(dt);
        }

        for ent in &mut self.children {
            ent.update(dt);
        }
    }

    pub fn render(&self, renderer: &Renderer) {
        for cmp in &self.components {
            cmp.render(renderer);
        }

        for ent in &self.children {
            ent.render(renderer);
        }
    }
}
