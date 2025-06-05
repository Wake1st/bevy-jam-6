use bevy::prelude::*;
use bevy_egui::egui::emath::inverse_lerp;

use crate::types::{
    energy::{Energy, START_AMOUNT},
    hub::Hub,
};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, adjust_mask);
    }
}

#[derive(Default, Debug, Component)]
pub struct PulsingMask;

fn adjust_mask(
    mut masks: Query<(&ChildOf, &mut Sprite), With<PulsingMask>>,
    hubs: Query<&Energy, With<Hub>>,
) {
    for (child, mut sprite) in masks.iter_mut() {
        let Ok(energy) = hubs.get(child.parent()) else {
            continue;
        };

        // set the alpha based on the age ratio
        if let Some(weight) = inverse_lerp(0.0..=START_AMOUNT, energy.amount) {
            sprite.color.set_alpha(weight);
        }
    }
}
