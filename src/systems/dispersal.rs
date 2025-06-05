use bevy::{math::FloatPow, prelude::*};

use crate::{
    systems::{
        audio::{QueueSFX, SFX, WAVE_SFX_PATH},
        pulse::PulseEvent,
        relationships::HubHolder,
    },
    theme::palette::ENERGY_COLOR,
    types::{
        energy::{
            BEAM_THICCNESS, BOLT_RADIUS, Energy, FIELD_RADIUS_RATIO, WAVE_RADIUS, WAVE_THICCNESS,
            Wave, spawn_energy_type,
        },
        hub::Hub,
        module::{Gong, Module, ModuleVarient},
        sounds::WaveSfx,
    },
};

const WAVE_SPEED: f32 = 64.;
const STRENGTH_TO_RADIUS: f32 = 0.1;

pub struct DispersalPlugin;

impl Plugin for DispersalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (create_wave, spread_wave, destroy_wave));
    }
}

fn create_wave(
    mut pulse: EventReader<PulseEvent>,
    hubs: Query<(Entity, &HubHolder), With<Hub>>,
    modules: Query<(&Module, &GlobalTransform), With<Gong>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut queue_sfx: EventWriter<QueueSFX>,
) {
    for e in pulse.read() {
        let Ok((hub_entity, holder)) = hubs.get(e.hub) else {
            continue;
        };

        let Ok((module, transform)) = modules.get(holder.0) else {
            continue;
        };

        // create an energy type
        let position = transform.translation().xy();
        let material = materials.add(ENERGY_COLOR);
        let audio_source = asset_server.load(WAVE_SFX_PATH);
        match module.varient {
            ModuleVarient::Gong(strength) => {
                let id = commands
                    .spawn((
                        Name::new("Wave"),
                        spawn_energy_type(
                            position,
                            meshes.add(Annulus::new(WAVE_RADIUS - WAVE_THICCNESS, WAVE_RADIUS)),
                            material,
                            audio_source,
                        ),
                        Wave {
                            strength: strength * e.energy * module.multiplier,
                            radius: WAVE_RADIUS,
                            origin: position.extend(0.0),
                            source: hub_entity,
                        },
                        WaveSfx,
                    ))
                    .id();

                // play sfx
                queue_sfx.write(QueueSFX {
                    sfx: SFX::WAVE,
                    entity: id,
                });
            }
            _ => (),
        };
    }
}

fn spread_wave(time: Res<Time>, mut waves: Query<(&mut Transform, &mut Wave)>) {
    let delta = time.delta_secs();

    for (mut transform, mut wave) in waves.iter_mut() {
        let growth = delta * WAVE_SPEED;

        // make weaker
        wave.radius += growth;
        wave.strength -= STRENGTH_TO_RADIUS * growth;

        // make bigger
        let ratio = wave.radius / WAVE_RADIUS;
        transform.scale.x = ratio;
        transform.scale.y = ratio;
    }
}

fn destroy_wave(waves: Query<(Entity, &Wave)>, mut commands: Commands) {
    for (entity, wave) in waves.iter() {
        if wave.strength < 0.0 {
            // commands.entity(entity).despawn_related();
            commands.entity(entity).despawn();
        }
    }
}

// ModuleVarient::Generator(radius) => (
//     Name::new("Generator"),
//     spawn_energy_type(
//         position,
//         meshes.add(Circle::new(FIELD_RADIUS_RATIO * radius * energy.amount)),
//         material,
//     ),
//     Field {},
// ),
// ModuleVarient::Tesla(_) => (
//     Name::new("Tesla"),
//     spawn_energy_type(position, meshes.add(Circle::new(BOLT_RADIUS)), material),
//     Bolt {
//         count: module.level,
//     },
// ),
// ModuleVarient::Lazer(length) => (
//     Name::new("Lazer"),
//     spawn_energy_type(
//         position,
//         meshes.add(Capsule2d::new(BEAM_THICCNESS, length * energy.amount)),
//         material,
//     ),
//     Beam {
//         length: length * energy.amount,
//         angle: 0.0,
//     },
// ),
