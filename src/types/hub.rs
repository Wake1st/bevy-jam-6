use bevy::prelude::*;

use super::energy::Energy;

#[derive(Component)]
pub struct Hub {
    pub multiplier: f32,
}

#[derive(Component)]
pub struct CentralHub;

pub fn spawn_hub(position: Vec2, multiplier: f32, texture: Handle<Image>) -> impl Bundle {
    (
        Sprite::from_image(texture),
        Transform::from_translation(position.extend(1.0)),
        Hub { multiplier },
        Energy { ..default() },
    )
}
