use std::f32::consts::PI;

use bevy::prelude::*;

const START_OFFSET: f32 = 0.001;
const THETA_OFFSET: f32 = PI / 2.0;
const EDGE_WIDTH: f32 = 8.0;
const EDGE_COLOR_BG: Color = Color::Srgba(Srgba {
    red: 0.7843137254901961,
    green: 0.00784313725490196,
    blue: 0.30980392156862746,
    alpha: 1.0,
});
const EDGE_COLOR_FL: Color = Color::Srgba(Srgba {
    red: 1.0,
    green: 0.34509803921568627,
    blue: 0.37254901960784315,
    alpha: 1.0,
});

#[derive(Component)]
pub struct Edge {
    pub length: f32,
}

pub fn spawn_edge(start: Vec2, end: Vec2) -> impl Bundle {
    let true_vector: Vec2 = end - start;
    let faux_start: Vec2 = true_vector.normalize() * START_OFFSET;
    let midpoint: Vec2 = faux_start + (end - faux_start) / 2.0;
    let theta: f32 = faux_start.angle_to(end) - THETA_OFFSET;
    let distance = end.distance(faux_start);
    info!(
        "\ntrue: {:?}\tfaux: {:?}\tmid: {:?}\ttheta: {:?}\tdist: {:?}",
        true_vector, faux_start, midpoint, theta, distance
    );

    let transform =
        Transform::from_xyz(midpoint.x, midpoint.y, 0.).with_rotation(Quat::from_rotation_z(theta));

    (
        Sprite {
            color: EDGE_COLOR_BG,
            custom_size: Some(Vec2::new(EDGE_WIDTH, distance)),
            ..default()
        },
        transform,
        Edge { length: distance },
    )
}

#[derive(Component)]
pub struct Filler {
    pub fullness: f32,
}

pub fn spawn_filler(start: Vec2, end: Vec2) -> impl Bundle {
    let true_vector: Vec2 = end - start;
    let faux_start: Vec2 = true_vector.normalize() * START_OFFSET;
    let theta: f32 = faux_start.angle_to(end) - THETA_OFFSET;

    let transform = Transform::from_xyz(faux_start.x, faux_start.y, 0.0)
        .with_rotation(Quat::from_rotation_z(theta));

    (
        Sprite {
            color: EDGE_COLOR_FL,
            custom_size: Some(Vec2::new(EDGE_WIDTH, 0.0)),
            ..default()
        },
        transform,
        Filler { fullness: 0.0 },
    )
}
