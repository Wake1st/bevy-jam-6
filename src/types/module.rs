use bevy::prelude::*;

#[derive(Component)]
pub struct Module {
    pub level: u8,
    pub multiplier: f32,
}

#[derive(Component)]
pub struct Gong {
    pub strength: f32, // how long the wave lasts
}

#[derive(Component)]
pub struct Generator {
    pub radius: f32, // how large the field
}

#[derive(Component)]
pub struct Tesla {
    pub bounces: u8, // how many times the bolt ricochets
}

#[derive(Component)]
pub struct Lazer {
    pub count: u8, // how many beams
}
