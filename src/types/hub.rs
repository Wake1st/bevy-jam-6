use bevy::prelude::*;

use crate::dnd::drop::Dropable;

use super::energy::Energy;

const HUB_LAYER: f32 = 0.0;

#[derive(Component, Default)]
pub struct Hub {
    pub multiplier: f32,
}

#[derive(Component, Default)]
pub struct CentralHub;

pub fn spawn_hub(position: Vec2, multiplier: f32, texture: Handle<Image>) -> impl Bundle {
    (
        Name::new("Hub"),
        Sprite::from_image(texture),
        Transform::from_translation(position.extend(HUB_LAYER)),
        Hub { multiplier },
        Energy { ..default() },
        Dropable,
    )
}
