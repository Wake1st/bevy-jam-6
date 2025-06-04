use bevy::prelude::*;
use bevy_egui::egui::emath::easing::cubic_in;

use crate::{
    systems::collisions::CollisionEvent,
    types::{
        energy::{Energy, START_AMOUNT},
        hub::{CentralHub, Hub},
    },
};

use super::currency::{self, Currency, CurrencyAdjusted};

pub const PULSE_RATE: f32 = 1.0;

pub struct PulsePlugin;

impl Plugin for PulsePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CentralPulse>();
        app.add_event::<PulseEvent>();
        app.add_systems(Update, (run_pulse, read_pulse, run_heartbeat, drain_energy));
    }
}

#[derive(Resource, Default)]
pub struct CentralPulse {
    pub age: f32,
}

#[derive(Component, Default, Debug)]
pub struct Heartbeat {
    pub age: f32,
}

#[derive(Event, Debug)]
pub struct PulseEvent {
    pub hub: Entity,
    pub energy: f32,
}

fn run_pulse(
    time: Res<Time>,
    mut pulse: ResMut<CentralPulse>,
    hub: Query<Entity, With<CentralHub>>,
    mut pulsed: EventWriter<PulseEvent>,
    mut faux_collided: EventWriter<CollisionEvent>,
) {
    let Ok(entity) = hub.single() else {
        return;
    };

    pulse.age += time.delta_secs();

    if pulse.age > PULSE_RATE {
        pulse.age -= PULSE_RATE;
        pulsed.write(PulseEvent {
            hub: entity,
            energy: START_AMOUNT,
        });
        faux_collided.write(CollisionEvent {
            hub: entity,
            energy: START_AMOUNT,
        });
    }
}

fn read_pulse(
    mut reader: EventReader<CollisionEvent>,
    mut hubs: Query<(&mut Energy, &Hub)>,
    mut adjusted: EventWriter<CurrencyAdjusted>,
) {
    for e in reader.read() {
        let Ok((mut energy, hub)) = hubs.get_mut(e.hub) else {
            continue;
        };

        let added: f32 = e.energy * hub.multiplier;
        energy.amount += added;
        adjusted.write(CurrencyAdjusted {
            amount: added.floor() as i128,
        });
    }
}

fn run_heartbeat(
    time: Res<Time>,
    mut hubs: Query<(Entity, &Energy, &mut Heartbeat), With<Hub>>,
    mut writer: EventWriter<PulseEvent>,
) {
    for (entity, energy, mut heartbeat) in hubs.iter_mut() {
        // beat the heart
        heartbeat.age += time.delta_secs();
        if heartbeat.age > PULSE_RATE {
            heartbeat.age -= PULSE_RATE;
            writer.write(PulseEvent {
                hub: entity,
                energy: energy.amount,
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
