use bevy::prelude::*;
use network::NetworkPlugin;
use pulse::PulsePlugin;

pub mod network;
pub mod pulse;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((NetworkPlugin, PulsePlugin));
    }
}
