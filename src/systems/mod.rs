use bevy::prelude::*;

use currency::CurrencyPlugin;
use effects::EffectsPlugin;
use network::NetworkPlugin;
use pulse::PulsePlugin;
use shop::ShopPlugin;
use ui::UIPlugin;

pub mod currency;
pub mod effects;
pub mod network;
pub mod pulse;
pub mod shop;
pub mod ui;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EffectsPlugin,
            NetworkPlugin,
            PulsePlugin,
            CurrencyPlugin,
            ShopPlugin,
            UIPlugin,
        ));
    }
}
