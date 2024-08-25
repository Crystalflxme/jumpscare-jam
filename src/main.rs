mod environment;
mod character;

use bevy::prelude::*;
use avian3d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
        ))
        .add_systems(Startup, (
            environment::setup,
            character::setup,
        ))
        .add_systems(Update, (
            environment::window_system,
            character::character_system,
        ))
        .add_systems(
            PostUpdate,
            character::camera_system
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        )
        .run();
}
