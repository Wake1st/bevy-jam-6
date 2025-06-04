use bevy::{platform::collections::HashMap, prelude::*, render::primitives::Aabb};

use crate::{
    systems::{
        pulse::{PULSE_RATE, PulseEvent},
        relationships::{EnergyConnected, EnergyDisconnected, EnergyGiver, EnergyReceiver},
    },
    types::{energy::Wave, hub::Hub},
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_wave_collision, cycle_collision_timer));
    }
}

#[derive(Component, Debug)]
pub struct CollisionTimer {
    pub giver: Entity,
    pub timer: Timer,
}

/// Connects the energy type to the hub when is has collided the first time
fn check_wave_collision(
    waves: Query<(Entity, &GlobalTransform, &Wave, &EnergyGiver)>,
    mut hubs: Query<(Entity, &Aabb, &mut Hub)>,
    mut connected: EventWriter<EnergyConnected>,
    mut pulsed: EventWriter<PulseEvent>,
) {
    // register all possible collisions
    let mut collisions: HashMap<Entity, Entity> = HashMap::new();
    for (wave_entity, transform, wave, _) in waves.iter() {
        for (hub_entity, aabb, _) in hubs.iter() {
            let dist: f32 = transform.translation().distance(aabb.center.into());
            let min_bound: f32 = dist - aabb.half_extents.x;
            let max_bound: f32 = dist + aabb.half_extents.x;

            if min_bound < wave.radius && wave.radius < max_bound {
                collisions.insert(wave_entity, hub_entity);
            }
        }
    }

    // process collisions
    for (wave_entity, hub_entity) in collisions.iter() {
        // check if already existing
        let Ok((_, _, wave, giver)) = waves.get(*wave_entity) else {
            continue;
        };

        let Ok((_, _, mut hub)) = hubs.get_mut(*hub_entity) else {
            continue;
        };

        if !giver.0.contains(hub_entity) {
            connected.write(EnergyConnected {
                giver: *wave_entity,
                receiver: *hub_entity,
            });
            pulsed.write(PulseEvent {
                hub: *hub_entity,
                energy: wave.strength,
            });

            hub.collision_timers.push(CollisionTimer {
                giver: *wave_entity,
                timer: Timer::from_seconds(PULSE_RATE, TimerMode::Once),
            });
        }
    }
}

/// Disconnects the energy type from the hub when the time has finished
fn cycle_collision_timer(
    time: Res<Time>,
    mut hubs: Query<(Entity, &mut Hub)>,
    mut disconnected: EventWriter<EnergyDisconnected>,
) {
    let delta = time.delta_secs();
    for (entity, mut hub) in hubs.iter_mut() {
        // remove finished timers
        let mut finished: Vec<usize>;
        for (index, collision) in hub.collision_timers.iter().enumerate() {
            collision.timer.tick(delta);
            if collision.timer.just_finished() {
                finished.push(index);
                disconnected.write(EnergyDisconnected {
                    giver: collision.giver,
                    receiver: entity,
                });
            }
        }

        for index in finished.iter() {
            hub.collision_timers.remove(index);
        }
    }
}
