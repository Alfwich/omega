use crate::core::component::component::Component;
use crate::core::renderer::renderer::Renderer;

#[derive(Default)]
pub struct Entity {
    pub components: Vec<Box<dyn Component>>,
    pub children: Vec<Entity>,
}

impl Entity {
    pub fn update(&mut self, dt: f32) {
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
