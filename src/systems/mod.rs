use bevy::prelude::*;
use network::NetworkPlugin;

pub mod network;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NetworkPlugin);
    }
}
