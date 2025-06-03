use bevy::prelude::*;

pub const START_AMOUNT: f32 = 1.;

#[derive(Component, Default)]
pub struct Energy {
    pub amount: f32,
}

#[derive(Component)]
pub struct Wave {
    pub strength: f32,
    pub radius: f32,
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
    pub angle: f32,
} // Lazer
