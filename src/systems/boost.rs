use bevy::{
    math::bounding::{BoundingCircle, BoundingVolume},
    prelude::*,
};
use bevy_cursor::CursorLocation;

use crate::{
    systems::{beam_dispersal::BoostBeam, currency::Currency, relationships::HubHolder},
    types::module::Module,
};

const MULTIPLIER_INCREMENT: f32 = 0.1;
const BOOST_EXPONENT: f32 = 18.0;
const BOOST_MULTIPLIER: f32 = 8.0;
const HUB_RADIUS: f32 = 16.0;
const CURSOR_RADIUS: f32 = 1.0;

pub struct BoostPlugin;

impl Plugin for BoostPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Boosted>();
        app.add_systems(Update, (check_boost_selection, boost_hub));
    }
}

#[derive(Event, Debug)]
pub struct Boosted {
    pub hub: Entity,
}

fn check_boost_selection(
    buttons: Res<ButtonInput<MouseButton>>,
    cursor: Res<CursorLocation>,
    hubs: Query<(Entity, &GlobalTransform)>,
    mut boosted: EventWriter<Boosted>,
) {
    // check for right click
    if !buttons.just_pressed(MouseButton::Right) {
        return;
    }

    // setup small circle for cursor
    let Some(position) = cursor.world_position() else {
        return;
    };
    let selection = BoundingCircle::new(position, CURSOR_RADIUS);

    for (entity, transform) in hubs.iter() {
        // check if hub selected
        let bounds = BoundingCircle::new(transform.translation().xy(), HUB_RADIUS);
        if !bounds.contains(&selection) {
            continue;
        }

        boosted.write(Boosted { hub: entity });
    }
}

fn boost_hub(
    mut boosted: EventReader<Boosted>,
    holders: Query<&HubHolder>,
    mut modules: Query<&mut Module>,
    mut currency: ResMut<Currency>,
    mut boost_beam: EventWriter<BoostBeam>,
) {
    for e in boosted.read() {
        let Ok(holder) = holders.get(e.hub) else {
            continue;
        };

        let Ok(mut module) = modules.get_mut(holder.0) else {
            continue;
        };

        // ensure there's money first
        let cost = (BOOST_MULTIPLIER * module.multiplier.powf(BOOST_EXPONENT)) as i128;
        if currency.0 > cost {
            currency.0 -= cost;

            module.level += 1;
            module.multiplier += MULTIPLIER_INCREMENT;

            boost_beam.write(BoostBeam {
                module: holder.0,
                boost: module.multiplier,
            });
        }
    }
}
