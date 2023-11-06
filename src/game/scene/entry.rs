use crate::{
    app::App,
    core::entity::entity::{Entity, EntityFns},
};

use super::testbed::make_testbed;

pub fn make_entry(app: &mut App) -> Entity {
    let mut e = Entity::new("entry", EntityFns::default());

    // Testbed
    e.add_child(make_testbed(app));

    e
}
