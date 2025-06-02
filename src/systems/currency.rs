use bevy::prelude::*;

pub const PRICE_GONG: i128 = 10;
pub const PRICE_GENERATOR: i128 = 120;
pub const PRICE_TESLA: i128 = 2100;
pub const PRICE_LAZER: i128 = 14000;

pub struct CurrencyPlugin;

impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Currency>();
        app.add_event::<CurrencyAdjusted>();

        app.add_systems(Update, adjust_currency);
    }
}

#[derive(Resource, Default, Debug)]
pub struct Currency(pub i128);

#[derive(Event, Debug)]
pub struct CurrencyAdjusted {
    pub amount: i128,
}

fn adjust_currency(mut event: EventReader<CurrencyAdjusted>, mut currency: ResMut<Currency>) {
    for e in event.read() {
        currency.0 += e.amount;
        info!(currency = ?currency.0);
    }
}
