use bevy::prelude::*;
use bevy_egui::egui::emath::easing::cubic_in;

use crate::types::{
    energy::Energy,
    hub::{CentralHub, Hub},
};

use super::currency::{self, Currency, CurrencyAdjusted};

const RATE: f32 = 1.0;
const AMOUNT: f32 = 1.0;

pub struct PulsePlugin;

impl Plugin for PulsePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Pulse>();
        app.add_event::<PulseEvent>();
        app.add_systems(Update, (run_pulse, begin_reaction, drain_energy));
    }
}

#[derive(Resource, Default)]
pub struct Pulse {
    pub age: f32,
}

#[derive(Event)]
pub struct PulseEvent;

fn run_pulse(time: Res<Time>, mut pulse: ResMut<Pulse>, mut writer: EventWriter<PulseEvent>) {
    pulse.age += time.delta_secs();

    if pulse.age > RATE {
        pulse.age -= RATE;
        writer.write(PulseEvent {});
    }
}

fn begin_reaction(
    mut reader: EventReader<PulseEvent>,
    mut central: Query<&mut Energy, With<CentralHub>>,
    mut adjusted: EventWriter<CurrencyAdjusted>,
) {
    for _ in reader.read() {
        if let Ok(mut energy) = central.single_mut() {
            energy.amount += AMOUNT;
            adjusted.write(CurrencyAdjusted {
                amount: AMOUNT.floor() as i128,
            });
        }
    }
}

fn drain_energy(time: Res<Time>, mut hubs: Query<&mut Energy, With<Hub>>) {
    let delta = time.delta_secs();

    for mut energy in hubs.iter_mut() {
        if energy.amount > 0.0 {
            energy.amount -= delta;
            if energy.amount < 0.0 {
                energy.amount = 0.0;
            }
        }
    }
}
