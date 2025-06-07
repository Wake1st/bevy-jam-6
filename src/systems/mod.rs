use bevy::prelude::*;

use currency::CurrencyPlugin;
use effects::EffectsPlugin;
use network::NetworkPlugin;
use pulse::PulsePlugin;
use shop::ShopPlugin;
use ui::UIPlugin;

use crate::systems::{
    audio::SfxPlugin, beam_dispersal::BeamDispersalPlugin, boost::BoostPlugin,
    collisions::CollisionPlugin, relationships::RelationshipPlugin,
    wave_dispersal::WaveDispersalPlugin,
};

pub mod audio;
pub mod beam_dispersal;
pub mod boost;
pub mod collisions;
pub mod currency;
pub mod effects;
pub mod network;
pub mod pulse;
pub mod relationships;
pub mod shop;
pub mod ui;
pub mod wave_dispersal;
pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            NetworkPlugin,
            PulsePlugin,
            RelationshipPlugin,
            WaveDispersalPlugin,
            BeamDispersalPlugin,
            CollisionPlugin,
            BoostPlugin,
        ))
        .add_plugins((
            CurrencyPlugin,
            EffectsPlugin,
            ShopPlugin,
            UIPlugin,
            SfxPlugin,
        ));
    }
}
