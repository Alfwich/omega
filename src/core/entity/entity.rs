use crate::app::App;
use crate::core::component::component::Component;
use crate::core::event::Event;
use crate::core::renderer::renderer::Renderer;

pub struct EntityFns {
    pub update_fn: fn(&mut Entity, &App, f32),
    pub event_fn: fn(&mut Entity, &mut App, &Event),
}

impl Default for EntityFns {
    fn default() -> Self {
        EntityFns {
            update_fn: |_e, _a, _d| {},
            event_fn: |_e, _a, _ev| {},
        }
    }
}

#[derive(Default)]
pub struct Entity {
    pub name: String,
    pub x: f32,
    pub y: f32,
    components: Vec<Box<dyn Component>>,
    children: Vec<Entity>,
    vtable: EntityFns,
}

impl Entity {
    pub fn new(name: &str, vtable: EntityFns) -> Self {
        Entity {
            name: name.to_string(),
            x: 0.,
            y: 0.,
            components: Vec::default(),
            children: Vec::default(),
            vtable,
        }
    }

    pub fn add_component<T: Component + 'static>(&mut self, cmp: T) {
        self.components.push(Box::new(cmp));
    }

    pub fn add_child(&mut self, ent: Entity) {
        self.children.push(ent);
    }

    pub fn find_component<T: Component + 'static>(&mut self, name: &str) -> Result<&mut T, String> {
        match self.find_component_by_name(name)?.as_any().downcast_mut() {
            Some(typed_cmp) => Ok(typed_cmp),
            None => Err(format!("Could not find component with name: {}", name)),
        }
    }

    pub fn find_component_by_name(
        &mut self,
        name: &str,
    ) -> Result<&mut Box<dyn Component>, String> {
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

    pub fn handle_event(&mut self, a: &mut App, e: &Event) {
        (self.vtable.event_fn)(self, a, e);

        for ent in &mut self.children {
            ent.handle_event(a, e);
        }
    }

    pub fn update(&mut self, app: &App, dt: f32) {
        (self.vtable.update_fn)(self, app, dt);

        for ent in &mut self.children {
            ent.update(app, dt);
        }
    }

    pub fn render_components(&self, renderer: &mut Renderer) {
        renderer.push_offset((self.x, self.y));
        for cmp in &self.components {
            cmp.render(renderer);
        }

        for ent in &self.children {
            ent.render_components(renderer);
        }
        renderer.pop_offset();
    }
}
