use std::time::Duration;

use bevy::{platform::collections::HashMap, prelude::*, render::primitives::Aabb};

use crate::{
    systems::pulse::{PULSE_RATE, PulseEvent},
    types::{
        energy::Wave,
        hub::{self, Hub},
    },
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();
        app.add_systems(Update, (check_wave_collision, cycle_collision_timer));
    }
}

#[derive(Component, Debug)]
pub struct CollisionTimer {
    pub giver: Entity,
    pub timer: Timer,
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub hub: Entity,
    pub energy: f32,
}

/// Connects the energy type to the hub when is has collided the first time
fn check_wave_collision(
    waves: Query<(Entity, &GlobalTransform, &Wave)>,
    mut hubs: Query<(Entity, &Aabb, &mut Hub)>,
    mut collided: EventWriter<CollisionEvent>,
) {
    // register all possible collisions
    let mut collisions: HashMap<Entity, Entity> = HashMap::new();
    for (wave_entity, transform, wave) in waves.iter() {
        for (hub_entity, aabb, _) in hubs.iter() {
            // don't check the sender
            if hub_entity == wave.source {
                continue;
            }

            // checking distance for collision
            let dist: f32 = transform.translation().distance(aabb.center.into());
            let min_bound: f32 = dist - aabb.half_extents.x;
            let max_bound: f32 = dist + aabb.half_extents.x;
            // info!(
            //     "{:?} -> {:?} | {:?} < {:?} < {:?}",
            //     wave_entity, hub_entity, min_bound, wave.radius, max_bound
            // );

            if min_bound < wave.radius && wave.radius < max_bound {
                // info!("{:?} collided with {:?}", wave_entity, hub_entity);
                collisions.insert(wave_entity, hub_entity);
            }
        }
    }

    // process collisions
    for (wave_entity, hub_entity) in collisions.iter() {
        let Ok((_, _, wave)) = waves.get(*wave_entity) else {
            continue;
        };

        let Ok((_, _, mut hub)) = hubs.get_mut(*hub_entity) else {
            continue;
        };

        // find existing timers
        if let Some(contains) = hub
            .collision_timers
            .iter()
            .find(|mut t| t.giver == *wave_entity)
        {
            // info!(
            //     "{:?} is already colliding with {:?}",
            //     wave_entity, hub_entity
            // );
            continue;
        }

        // add new timer and give energy to hub
        hub.collision_timers.push(CollisionTimer {
            giver: *wave_entity,
            timer: Timer::from_seconds(PULSE_RATE, TimerMode::Once),
        });

        collided.write(CollisionEvent {
            hub: *hub_entity,
            energy: wave.strength,
        });
    }
}

/// Disconnects the energy type from the hub when the time has finished
fn cycle_collision_timer(time: Res<Time>, mut hubs: Query<(Entity, &mut Hub)>) {
    let delta = time.delta_secs();
    for (entity, mut hub) in hubs.iter_mut() {
        // remove finished timers
        let mut finished: Vec<usize> = vec![];
        for (index, mut collision) in hub.collision_timers.iter_mut().enumerate() {
            collision.timer.tick(Duration::from_secs_f32(delta));
            if collision.timer.just_finished() {
                finished.push(index);
            }
        }

        for &index in finished.iter() {
            hub.collision_timers.remove(index);
        }
    }
}
