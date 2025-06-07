use std::f32::consts::PI;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    game::reset_game,
    systems::{
        // audio::HUB_SFX_PATH,
        pulse::Heartbeat,
    },
    types::{
        hub::{CentralHub, Hub, spawn_hub, spawn_hub_mask},
        module::Module,
    },
};

const LAYER_COUNT: u8 = 8;
const LAYER_THICKNESS: f32 = 80.0;
const LAYER_MULTIPLIER: f32 = 0.38;
const ANGLE_OFFSET: f32 = 0.3;
const CART_OFFSET: f32 = 12.0;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_hub_map).add_systems(
            Update,
            ((clear_modules, clear_hub_map), generate_hub_map)
                .chain()
                .run_if(reset_game),
        );
    }
}

fn generate_hub_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let hub_texture = asset_server.load("images/hub.png");
    let mask_texture = asset_server.load("images/hub_mask.png");
    // let audio_source = asset_server.load(HUB_SFX_PATH);
    let mut rng = rand::rng();
    let mut layer_multiplier = 1.0;

    // starting hub
    commands.spawn((
        spawn_hub(
            Vec2 { x: 0.0, y: 0.0 },
            layer_multiplier,
            hub_texture.clone(),
            // audio_source.clone(),
        ),
        CentralHub,
        children![spawn_hub_mask(mask_texture.clone())],
    ));

    // spawn layers of hubs
    for l in 1..LAYER_COUNT {
        layer_multiplier *= l as f32 * LAYER_MULTIPLIER;

        let radius: f32 = LAYER_THICKNESS * rng.random_range(l..(l + 1)) as f32;
        let hubs_per_layer: u8 = rng.random_range(l..((l + 1) * 2));
        let slice_angle: f32 = 2.0 * PI / hubs_per_layer as f32;

        for n in 0..hubs_per_layer {
            // use a mirror symmetry
            let range = rng.random_range(-ANGLE_OFFSET..ANGLE_OFFSET);
            let theta: f32 = n as f32 * slice_angle + range;

            let x = radius * f32::cos(theta) + rng.random_range(-CART_OFFSET..CART_OFFSET);
            let y = radius * f32::sin(theta) + rng.random_range(-CART_OFFSET..CART_OFFSET);

            commands.spawn((
                spawn_hub(
                    Vec2 { x, y },
                    layer_multiplier,
                    hub_texture.clone(),
                    // audio_source.clone(),
                ),
                Heartbeat { ..default() },
                children![spawn_hub_mask(mask_texture.clone())],
            ));
        }
    }
}

fn clear_hub_map(mut commands: Commands, hubs: Query<Entity, With<Hub>>) {
    for hub in hubs.iter() {
        commands.entity(hub).despawn();
    }
}

fn clear_modules(mut commands: Commands, modules: Query<Entity, With<Module>>) {
    for module in modules.iter() {
        commands.entity(module).despawn();
    }
}
