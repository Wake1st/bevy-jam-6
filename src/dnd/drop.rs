use bevy::prelude::*;
use bevy_cursor::CursorLocation;

use crate::{
    ScheduleSystems,
    systems::{
        currency::{
            CurrencyAdjusted,
            PRICE_GONG,
            // PRICE_GENERATOR, PRICE_LAZER, PRICE_TESLA
        },
        relationships::ModuleAttached,
    },
    types::module::{Module, ModuleVarient},
};

use super::drag::Dragging;

const DROPABLE_SIZE: Vec2 = Vec2::splat(32.0);

pub(super) fn plugin(app: &mut App) {
    app.add_event::<Released>();
    app.add_systems(Update, drop.in_set(ScheduleSystems::RecordInput))
        .add_systems(Update, released);
}

#[derive(Component, Debug)]
pub struct Dropable;

fn drop(
    buttons: Res<ButtonInput<MouseButton>>,
    mut dragging: Query<(Entity, &mut Transform), With<Dragging>>,
    dropables: Query<(Entity, &GlobalTransform), With<Dropable>>,
    cursor: Res<CursorLocation>,
    mut commands: Commands,
    mut released: EventWriter<Released>,
    mut attached: EventWriter<ModuleAttached>,
) {
    // Only end on mouse up
    if !buttons.just_released(MouseButton::Left) {
        return;
    }

    // Only run if dragging exist
    if let Ok((dragging_entity, mut dragging_transform)) = dragging.single_mut() {
        // checking if dropped anywhere
        let mut dropped: bool = false;

        // Check for possible collision
        for (dropable_entity, dropable_transform) in dropables.iter() {
            let dropable_position = dropable_transform.translation().xy();
            let rect = Rect::from_center_size(dropable_position, DROPABLE_SIZE);

            // we need the cursor position to find a match
            let Some(position) = cursor.world_position() else {
                return;
            };

            if rect.contains(position) {
                // snap draggable to droppable
                dragging_transform.translation = dropable_position.extend(1.0);

                // no longer dragging
                commands.entity(dragging_entity).remove::<Dragging>();

                // connect the two
                attached.write(ModuleAttached {
                    hub: dropable_entity,
                    module: dragging_entity,
                });

                // exit to ensure this process happens ONCE
                dropped = true;
                break;
            }
        }

        // destroy it if we dropped into nothing
        if !dropped {
            released.write(Released {
                entity: dragging_entity,
            });
        }
    }
}

#[derive(Event, Debug)]
pub struct Released {
    entity: Entity,
}

fn released(
    mut released: EventReader<Released>,
    mut commands: Commands,
    modules: Query<&Module>,
    mut currency_adjustment: EventWriter<CurrencyAdjusted>,
) {
    for e in released.read() {
        // remove
        commands.entity(e.entity).despawn();

        // refund
        if let Ok(module) = modules.get(e.entity) {
            currency_adjustment.write(CurrencyAdjusted {
                amount: match module.varient {
                    ModuleVarient::Gong(_) => PRICE_GONG,
                    // ModuleVarient::Generator(_) => PRICE_GENERATOR,
                    // ModuleVarient::Tesla(_) => PRICE_TESLA,
                    // ModuleVarient::Lazer(_) => PRICE_LAZER,
                },
            });
        }
    }
}
