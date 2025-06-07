use bevy::prelude::*;

use crate::dnd::drag::{Draggable, Dragging};

pub const BASE_STRENGTH: f32 = 10.0;
// pub const BASE_RADIUS: f32 = 40.0;
// pub const BASE_BOUNCE: u8 = 1;
pub const BASE_LENGTH: f32 = 240.0;

const MODULE_LAYER: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Module {
    pub level: u8,
    pub multiplier: f32,
    pub varient: ModuleVarient,
}

#[derive(Debug, Clone)]
pub enum ModuleVarient {
    Gong(f32),
    // Generator(f32),
    // Tesla(u8),
    Lazer(f32),
}

#[derive(Reflect, Component, Default, Debug, Clone)]
#[reflect(Component)]
pub struct Gong;
// pub strength: f32, // how long the wave lasts
// }

// #[derive(Reflect, Component, Default, Debug, Clone)]
// #[reflect(Component)]
// pub struct Generator;
// // pub radius: f32, // how large the field
// // }

// #[derive(Reflect, Component, Default, Debug, Clone)]
// #[reflect(Component)]
// pub struct Tesla;
// // pub bounces: u8, // how many times the bolt ricochets
// // }

#[derive(Reflect, Component, Default, Debug, Clone)]
#[reflect(Component)]
pub struct Lazer;
// // pub count: u8, // how many beams
// // }

pub fn spawn_gong(position: Vec2, texture: Handle<Image>) -> impl Bundle {
    (
        Name::new("Gong"),
        Sprite::from_image(texture),
        Transform::from_translation(position.extend(MODULE_LAYER)),
        Module {
            level: 1,
            multiplier: 1.0,
            varient: ModuleVarient::Gong(BASE_STRENGTH),
        },
        Gong,
        Draggable,
        Dragging,
    )
}

// pub fn spawn_generator(position: Vec2, texture: Handle<Image>) -> impl Bundle {
//     (
//         Name::new("Generator"),
//         Sprite::from_image(texture),
//         Transform::from_translation(position.extend(MODULE_LAYER)),
//         Module {
//             level: 1,
//             multiplier: 1.0,
//             varient: ModuleVarient::Generator(BASE_RADIUS),
//         },
//         Generator,
//         Draggable,
//         Dragging,
//     )
// }

pub fn spawn_lazer(position: Vec2, texture: Handle<Image>) -> impl Bundle {
    (
        Name::new("Lazer"),
        Sprite::from_image(texture),
        Transform::from_translation(position.extend(MODULE_LAYER)),
        Module {
            level: 1,
            multiplier: 1.0,
            varient: ModuleVarient::Lazer(BASE_LENGTH),
        },
        Lazer,
        Draggable,
        Dragging,
    )
}

// pub fn spawn_tesla(position: Vec2, texture: Handle<Image>) -> impl Bundle {
//     (
//         Name::new("Tesla"),
//         Sprite::from_image(texture),
//         Transform::from_translation(position.extend(MODULE_LAYER)),
//         Module {
//             level: 1,
//             multiplier: 1.0,
//             varient: ModuleVarient::Tesla(BASE_BOUNCE),
//         },
//         Tesla,
//         Draggable,
//         Dragging,
//     )
// }
