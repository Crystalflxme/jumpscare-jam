mod environment;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            environment::EnvironmentPlugin
        )).run();
}
