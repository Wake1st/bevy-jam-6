use bevy::prelude::*;

#[derive(Component)]
pub struct Hub {
    pub multiplier: f32,
}

pub fn spawn_hub(position: Vec2, multiplier: f32, texture: Handle<Image>) -> impl Bundle {
    (
        Sprite::from_image(texture),
        Transform::from_translation(position.extend(1.0)),
        Hub { multiplier },
    )
}
