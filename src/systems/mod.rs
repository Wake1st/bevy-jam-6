use bevy::prelude::*;

use currency::CurrencyPlugin;
use network::NetworkPlugin;
use pulse::PulsePlugin;
use shop::ShopPlugin;
use ui::UIPlugin;

pub mod currency;
pub mod network;
pub mod pulse;
pub mod shop;
pub mod ui;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            NetworkPlugin,
            PulsePlugin,
            CurrencyPlugin,
            ShopPlugin,
            UIPlugin,
        ));
    }
}
