use bevy::{prelude::*, render::primitives::Aabb};

use crate::{
    dnd::drop::Dropable,
    systems::{collisions::CollisionTimer, effects::PulsingMask},
    types::sounds::HubSfx,
};

use super::energy::Energy;

const HUB_LAYER: f32 = 0.0;
const HUB_HALF_EXTENDS: Vec3A = Vec3A::new(1., 1., 0.);

#[derive(Component, Default)]
pub struct Hub {
    pub collision_timers: Vec<CollisionTimer>,
    pub multiplier: f32,
}

#[derive(Component, Default)]
pub struct CentralHub;

pub fn spawn_hub(
    position: Vec2,
    multiplier: f32,
    texture: Handle<Image>,
    source: Handle<AudioSource>,
) -> impl Bundle {
    (
        Name::new("Hub"),
        Sprite::from_image(texture),
        Aabb {
            center: position.extend(0.0).into(),
            half_extents: HUB_HALF_EXTENDS,
        },
        Transform::from_translation(position.extend(HUB_LAYER)),
        Hub {
            multiplier,
            ..default()
        },
        Energy { ..default() },
        Dropable,
        HubSfx,
        AudioPlayer::new(source),
    )
}

pub fn spawn_hub_mask(texture: Handle<Image>) -> impl Bundle {
    (
        Name::new("Mask"),
        PulsingMask { ..default() },
        Sprite::from_image(texture),
    )
}
