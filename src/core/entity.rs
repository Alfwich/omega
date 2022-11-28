use crate::core::component::component::Component;
use crate::core::renderer::renderer::Renderer;

pub struct Entity {
    pub name: String,
    pub update_fn: fn(&mut Entity, dt: f32),
    pub components: Vec<Box<dyn Component>>,
    pub children: Vec<Entity>,
}

impl Entity {
    pub fn new(name: &str, update: fn(&mut Entity, dt: f32)) -> Self {
        Entity {
            name: name.to_string(),
            update_fn: update,
            components: Vec::default(),
            children: Vec::default(),
        }
    }

    pub fn find_component<T: Component + 'static>(&mut self, name: &str) -> Result<&mut T, String> {
        match self.find_component_by_name(name)?.as_any().downcast_mut() {
            Some(typed_cmp) => {
                Ok(typed_cmp)
            }
            None => {
                Err(format!("Could not find component with name: {}", name))
            }
        }
    }

    pub fn find_component_by_name(&mut self, name: &str) -> Result<&mut Box<dyn Component>, String> {
        for cmp in &mut self.components {
            let component = &*cmp;
            if component.get_name() == name {
                return Ok(cmp);
            }
        }
        
        Err(format!("Could not find component with name: {}", name))
    }
    
    pub fn find_child_by_name(&mut self, name: &str) -> Result<&mut Entity, String> {
        for child in &mut self.children {
            if child.name == name {
                return Ok(child);
            }
        }
        
        Err(format!("Could not find child with name: {}", name))
    }
    
    pub fn update(&mut self, dt: f32) {
        
        (self.update_fn)(self, dt);
        
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
