use crate::app::App;
use crate::core::component::component::Component;
use crate::core::component::offset::{Offset, OFFSET_NAME};
use crate::core::event::{Event, UpdateRenderablePayload};

pub struct EntityFns {
    pub update_fn: fn(&mut Entity, &App, f32),
    pub event_fn: fn(&mut Entity, &mut Option<&mut App>, &Event),
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
    components: Vec<Box<dyn Component>>,
    children: Vec<Entity>,
    vtable: EntityFns,
}

impl Entity {
    pub fn new(name: &str, vtable: EntityFns) -> Self {
        Entity {
            name: name.to_string(),
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

    pub fn handle_event(&mut self, a: &mut Option<&mut App>, e: &Event) {
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

    pub fn render_components(&mut self, app: &App, parent_offset: (f32, f32)) {
        let offset = match self.find_component::<Offset>(OFFSET_NAME) {
            Ok(offset) => (parent_offset.0 + offset.x, parent_offset.1 + offset.y),
            _ => (0., 0.),
        };

        for cmp in &self.components {
            cmp.render(app);
        }

        for ent in &mut self.children {
            ent.render_components(app, offset);
        }
    }
}

pub trait RenderableEntity {
    fn set_x(&mut self, x: f32);
    fn move_x(&mut self, mx: f32);
    fn set_y(&mut self, y: f32);
    fn move_y(&mut self, my: f32);
    fn set_width(&mut self, w: f32);
    fn set_height(&mut self, h: f32);
    fn set_rotation(&mut self, r: f32);
    fn set_scale_x(&mut self, sx: f32);
    fn set_scale_y(&mut self, sy: f32);
}

// Functions to support "renderable" entities
impl RenderableEntity for Entity {
    fn set_x(&mut self, x: f32) {
        (self.vtable.event_fn)(
            self,
            &mut None,
            &Event::UpdateRenderable(UpdateRenderablePayload::X(x)),
        );
    }

    fn move_x(&mut self, mx: f32) {
        (self.vtable.event_fn)(
            self,
            &mut None,
            &Event::UpdateRenderable(UpdateRenderablePayload::MoveX(mx)),
        );
    }

    fn set_y(&mut self, y: f32) {
        (self.vtable.event_fn)(
            self,
            &mut None,
            &Event::UpdateRenderable(UpdateRenderablePayload::Y(y)),
        );
    }

    fn move_y(&mut self, my: f32) {
        (self.vtable.event_fn)(
            self,
            &mut None,
            &Event::UpdateRenderable(UpdateRenderablePayload::MoveY(my)),
        );
    }

    fn set_width(&mut self, w: f32) {
        (self.vtable.event_fn)(
            self,
            &mut None,
            &Event::UpdateRenderable(UpdateRenderablePayload::Width(w)),
        );
    }

    fn set_height(&mut self, h: f32) {
        (self.vtable.event_fn)(
            self,
            &mut None,
            &Event::UpdateRenderable(UpdateRenderablePayload::Height(h)),
        );
    }

    fn set_rotation(&mut self, r: f32) {
        (self.vtable.event_fn)(
            self,
            &mut None,
            &Event::UpdateRenderable(UpdateRenderablePayload::Rotation(r)),
        );
    }

    fn set_scale_x(&mut self, sx: f32) {
        (self.vtable.event_fn)(
            self,
            &mut None,
            &Event::UpdateRenderable(UpdateRenderablePayload::ScaleX(sx)),
        );
    }

    fn set_scale_y(&mut self, sy: f32) {
        (self.vtable.event_fn)(
            self,
            &mut None,
            &Event::UpdateRenderable(UpdateRenderablePayload::ScaleY(sy)),
        );
    }
}
