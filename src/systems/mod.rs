use bevy::prelude::*;

use currency::CurrencyPlugin;
use effects::EffectsPlugin;
use network::NetworkPlugin;
use pulse::PulsePlugin;
use shop::ShopPlugin;
use ui::UIPlugin;

use crate::systems::{
    boost::BoostPlugin, collisions::CollisionPlugin, dispersal::DispersalPlugin,
    relationships::RelationshipPlugin,
};

pub mod boost;
pub mod collisions;
pub mod currency;
pub mod dispersal;
pub mod effects;
pub mod network;
pub mod pulse;
pub mod relationships;
pub mod shop;
pub mod ui;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            NetworkPlugin,
            PulsePlugin,
            RelationshipPlugin,
            DispersalPlugin,
            CollisionPlugin,
            BoostPlugin,
        ))
        .add_plugins((CurrencyPlugin, EffectsPlugin, ShopPlugin, UIPlugin));
    }
}
