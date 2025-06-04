use bevy::prelude::*;

pub const START_AMOUNT: f32 = 1.;

pub const WAVE_RADIUS: f32 = 12.;
pub const WAVE_THICCNESS: f32 = 0.6;
pub const FIELD_RADIUS_RATIO: f32 = 6.;
pub const BOLT_RADIUS: f32 = 24.;
pub const BEAM_THICCNESS: f32 = 16.;

const ENERGY_LAYER: f32 = 0.4;

#[derive(Component, Default)]
pub struct Energy {
    pub amount: f32,
}

#[derive(Component)]
pub struct Wave {
    pub strength: f32,
    pub radius: f32,
    pub origin: Vec3,
    pub source: Entity,
} // Gong

#[derive(Component)]
pub struct Field {
    pub radius: f32,
} // Generator

#[derive(Component)]
pub struct Bolt {
    pub bounces: f32,
} // Tesla

#[derive(Component)]
pub struct Beam {
    pub length: f32,
    pub angle: f32,
} // Lazer

pub fn spawn_energy_type(
    position: Vec2,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) -> impl Bundle {
    (
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_translation(position.extend(ENERGY_LAYER)),
    )
}
