use crate::{
    app::App,
    core::entity::{Entity, EntityFns},
};

use super::testbed::make_testbed;

pub fn make_entry(app: &mut App) -> Entity {
    let mut e = Entity::new("entry", EntityFns::default());

    // Testbed(s)
    let num_scenes = 5;
    for ts in 1..=num_scenes {
        e.add_child(make_testbed(app, (1. / num_scenes as f32) * ts as f32));
    }

    e
}
