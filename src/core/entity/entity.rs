use std::default;

use crate::app::App;
use crate::core::component::component::Component;
use crate::core::component::offset::{self, Offset, OFFSET_NAME};
use crate::core::event::{Event, UpdateRenderablePayload};
use crate::core::renderer::renderer::Renderer;

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

    pub fn render_components(&mut self, renderer: &mut Renderer) {
        let offset = match self.find_component::<Offset>(OFFSET_NAME) {
            Ok(offset) => (offset.x, offset.y),
            _ => (0., 0.),
        };

        // Push offset
        renderer.offset.0 += offset.0;
        renderer.offset.1 += offset.1;

        for cmp in &self.components {
            cmp.render(renderer);
        }

        for ent in &mut self.children {
            ent.render_components(renderer);
        }

        // Pop offset
        renderer.offset.0 -= offset.0;
        renderer.offset.1 -= offset.1;
    }

    // Functions to support "renderable" entities
    pub fn set_x(&mut self, x: f32) {
        let e = Event::UpdateRenderable(UpdateRenderablePayload {
            x: Some(x),
            ..Default::default()
        });

        (self.vtable.event_fn)(self, &mut None, &e);
    }

    pub fn set_y(&mut self, y: f32) {
        let e = Event::UpdateRenderable(UpdateRenderablePayload {
            y: Some(y),
            ..Default::default()
        });

        (self.vtable.event_fn)(self, &mut None, &e);
    }

    pub fn set_width(&mut self, w: f32) {
        let e = Event::UpdateRenderable(UpdateRenderablePayload {
            w: Some(w),
            ..Default::default()
        });

        (self.vtable.event_fn)(self, &mut None, &e);
    }

    pub fn set_height(&mut self, h: f32) {
        let e = Event::UpdateRenderable(UpdateRenderablePayload {
            h: Some(h),
            ..Default::default()
        });

        (self.vtable.event_fn)(self, &mut None, &e);
    }

    pub fn set_rotation(&mut self, r: f32) {
        let e = Event::UpdateRenderable(UpdateRenderablePayload {
            r: Some(r),
            ..Default::default()
        });

        (self.vtable.event_fn)(self, &mut None, &e);
    }

    pub fn set_scale_x(&mut self, sx: f32) {
        let e = Event::UpdateRenderable(UpdateRenderablePayload {
            scale_x: Some(sx),
            ..Default::default()
        });

        (self.vtable.event_fn)(self, &mut None, &e);
    }

    pub fn set_scale_y(&mut self, sy: f32) {
        let e = Event::UpdateRenderable(UpdateRenderablePayload {
            scale_y: Some(sy),
            ..Default::default()
        });

        (self.vtable.event_fn)(self, &mut None, &e);
    }
}
