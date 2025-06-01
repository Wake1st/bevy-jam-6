use bevy::prelude::*;
use systems::SystemsPlugin;

mod systems;
mod types;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SystemsPlugin)
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands) {
    // starting node
    commands.spawn(Camera2d);
}
