pub mod animated_image;

use crate::app::App;
use crate::core::component::offset::{Offset, OFFSET_NAME};
use crate::core::component::Component;
use crate::core::event::{Event, UpdateRenderablePayload};

pub struct EntityFns {
    /// Called once per frame with the deltatime from the previous frame
    pub update_fn: fn(&mut Entity, &App, f32),

    /// Event responder function
    pub event_fn: fn(&mut Entity, &mut Option<&mut App>, &Event),

    /// Called right before the Entitie's components are rendered
    pub prerender_fn: fn(&mut Entity, parent_offset: (f32, f32)),
}

impl Default for EntityFns {
    fn default() -> Self {
        EntityFns {
            update_fn: |_e, _a, _d| {},
            event_fn: |_e, _a, _ev| {},
            prerender_fn: |_e, _o| {},
        }
    }
}

/// Entities are composed of a combination of Components, and other Entities.
enum EntityChild {
    Component(Box<dyn Component>),
    Entity(Entity),
}

pub struct Entity {
    pub name: String,
    pub zindex: i32,
    pub active: bool,
    pub visible: bool,
    children: Vec<EntityChild>,
    vtable: EntityFns,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            zindex: 0,
            active: true,
            visible: true,
            children: Vec::default(),
            vtable: EntityFns::default(),
        }
    }
}

impl Entity {
    pub fn new(name: &str, vtable: EntityFns) -> Self {
        Entity {
            name: name.to_string(),
            zindex: 0,
            active: true,
            visible: true,
            children: Vec::default(),
            vtable,
        }
    }

    pub fn add_component<T: Component + 'static>(&mut self, cmp: T) {
        self.children.push(EntityChild::Component(Box::new(cmp)));
    }

    pub fn add_child(&mut self, ent: Entity) {
        self.children.push(EntityChild::Entity(ent));
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
        for c in &mut self.children {
            if let EntityChild::Component(cmp) = c {
                let component = &*cmp;
                if component.get_name() == name {
                    return Ok(cmp);
                }
            }
        }

        Err(format!("Could not find component with name: {}", name))
    }

    pub fn find_child_by_name(&mut self, name: &str) -> Result<&mut Entity, String> {
        for c in &mut self.children {
            if let EntityChild::Entity(ent) = c {
                if ent.name == name {
                    return Ok(ent);
                }
            }
        }

        Err(format!("Could not find child with name: {}", name))
    }

    pub fn handle_event(&mut self, a: &mut Option<&mut App>, e: &Event) {
        if self.active {
            (self.vtable.event_fn)(self, a, e);

            for c in &mut self.children {
                if let EntityChild::Entity(ent) = c {
                    ent.handle_event(a, e);
                }
            }
        }
    }

    /// Reorder children based on z_index value. This will recursively reorder the entire Entity tree.
    /// Parent z_index ordering will determine the final outcome where z_index does not impact global ordering.
    pub fn reorder_children(&mut self) {
        self.children.sort_by(|a, b| {
            let av = match a {
                EntityChild::Entity(ent) => ent.z_index(),
                EntityChild::Component(cmp) => cmp.z_index(),
            };

            let bv = match b {
                EntityChild::Entity(ent) => ent.z_index(),
                EntityChild::Component(cmp) => cmp.z_index(),
            };

            av.partial_cmp(&bv).unwrap()
        });

        // Recursive call to reorder whole graph
        for c in &mut self.children {
            if let EntityChild::Entity(ent) = c {
                ent.reorder_children();
            }
        }
    }

    pub fn update(&mut self, app: &App, dt: f32) {
        if self.active {
            (self.vtable.update_fn)(self, app, dt);

            for c in &mut self.children {
                if let EntityChild::Entity(ent) = c {
                    ent.update(app, dt);
                }
            }
        }
    }

    fn z_index(&self) -> i32 {
        self.zindex
    }

    pub fn render_components(&mut self, app: &App, parent_offset: (f32, f32)) {
        if self.visible {
            let offset = match self.find_component::<Offset>(OFFSET_NAME) {
                Ok(offset) => (parent_offset.0 + offset.x, parent_offset.1 + offset.y),
                _ => parent_offset,
            };

            (self.vtable.prerender_fn)(self, offset);

            for c in &mut self.children {
                match c {
                    EntityChild::Entity(ent) => {
                        ent.render_components(app, offset);
                    }

                    EntityChild::Component(cmp) => {
                        cmp.render(app, offset);
                    }
                }
            }
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
    fn set_alpha(&mut self, a: f32);
    fn set_color_mod(&mut self, r: f32, g: f32, b: f32);
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

    fn set_alpha(&mut self, a: f32) {
        (self.vtable.event_fn)(
            self,
            &mut None,
            &Event::UpdateRenderable(UpdateRenderablePayload::Alpha(a)),
        );
    }

    fn set_color_mod(&mut self, r: f32, g: f32, b: f32) {
        (self.vtable.event_fn)(
            self,
            &mut None,
            &Event::UpdateRenderable(UpdateRenderablePayload::ColorMod(r, g, b)),
        );
    }
}
