use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    game::reset_game,
    systems::relationships::{LazerBeam, ModuleAttached, ModuleRemoved},
    theme::palette::ENERGY_COLOR,
    types::{
        energy::{BEAM_CYCLE_SECS, BEAM_LENGTH, BEAM_THICCNESS, Beam, ENERGY_LAYER},
        module::{Lazer, Module, ModuleVarient},
    },
};

pub struct BeamDispersalPlugin;

impl Plugin for BeamDispersalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BoostBeam>();
        app.add_systems(
            Update,
            (
                create_beam,
                age_beam,
                rotate_beam,
                boost_beam,
                remove_beam,
                clear_beams.run_if(reset_game),
            ),
        );
    }
}

#[derive(Component, Debug)]
pub struct BeamRay;

fn create_beam(
    mut module_attached: EventReader<ModuleAttached>,
    modules: Query<(&Module, &GlobalTransform), With<Lazer>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for e in module_attached.read() {
        let Ok((module, transform)) = modules.get(e.module) else {
            continue;
        };

        // create an energy type
        let position = transform.translation().xy();
        let material = materials.add(ENERGY_COLOR);
        let ModuleVarient::Lazer(length) = module.varient else {
            continue;
        };
        let transform = Transform::from_xyz(length / 2., 0., 0.).rotate_z(PI / 2.);

        let id = commands
            .spawn((
                Name::new("Beam"),
                Beam { ..default() },
                Transform::from_translation(position.extend(ENERGY_LAYER)),
                children![(
                    Name::new("Beam Ray"),
                    BeamRay,
                    Mesh2d(meshes.add(Capsule2d::new(BEAM_THICCNESS, length))),
                    MeshMaterial2d(material),
                    transform,
                )],
            ))
            .id();

        // attach beam to module
        commands.entity(e.module).insert(LazerBeam(id));
    }
}

fn age_beam(time: Res<Time>, mut beams: Query<&mut Beam>) {
    let delta = time.delta_secs();
    for mut beam in beams.iter_mut() {
        // update and loop around age
        beam.age += delta;
        if beam.age > BEAM_CYCLE_SECS {
            beam.age -= BEAM_CYCLE_SECS;
        }

        // angle is age based
        beam.angle = (2. * PI) * (beam.age / BEAM_CYCLE_SECS);
    }
}

fn rotate_beam(mut beams: Query<(&mut Transform, &Beam)>) {
    for (mut transfrom, beam) in beams.iter_mut() {
        transfrom.rotation.z = beam.angle;
    }
}

#[derive(Event, Debug)]
pub struct BoostBeam {
    pub module: Entity,
    pub boost: f32,
}

fn boost_beam(
    mut boosted: EventReader<BoostBeam>,
    modules: Query<&LazerBeam, With<Module>>,
    beams: Query<&Children, With<Beam>>,
    mut rays: Query<&mut Transform, With<BeamRay>>,
) {
    for e in boosted.read() {
        let Ok(lazer_beam) = modules.get(e.module) else {
            continue;
        };

        let Ok(children) = beams.get(lazer_beam.0) else {
            continue;
        };

        for child in children.iter() {
            let Ok(mut transform) = rays.get_mut(child) else {
                continue;
            };

            info!("boosted beam?");
            transform.translation.x = (BEAM_LENGTH * e.boost) / 2.;
            transform.scale.x = e.boost;
        }
    }
}

fn remove_beam(mut removed: EventReader<ModuleRemoved>, mut commands: Commands) {
    for e in removed.read() {
        commands.entity(e.module).remove::<LazerBeam>();
    }
}

fn clear_beams(beams: Query<Entity, With<Beam>>, mut commands: Commands) {
    for beam in beams.iter() {
        commands.entity(beam).despawn();
    }
}
