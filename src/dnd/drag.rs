use bevy::prelude::*;
use bevy_cursor::CursorLocation;

use crate::{AppSet, systems::relationships::ModuleRemoved};

const DRAGGABLE_SIZE: Vec2 = Vec2::splat(32.0);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            start_drag.in_set(AppSet::RecordInput),
            drag.in_set(AppSet::Update),
        ),
    );
}

#[derive(Component, Debug)]
pub struct Draggable;

#[derive(Component, Debug)]
pub struct Dragging;

fn start_drag(
    dragging: Query<&Dragging>,
    draggables: Query<(Entity, &GlobalTransform), With<Draggable>>,
    cursor: Res<CursorLocation>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut removed: EventWriter<ModuleRemoved>,
) {
    // Only start on mouse down
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    // ensure we have a cursor
    if let Some(position) = cursor.world_position() {
        // Check any dragging exist and return early if true
        if dragging.iter().next().is_some() {
            return;
        }

        // Check for possible collision
        for (entity, transform) in draggables.iter() {
            let rect = Rect::from_center_size(transform.translation().xy(), DRAGGABLE_SIZE);
            if rect.contains(position) {
                // add dragging component to the word
                commands.entity(entity).insert(Dragging);
                removed.write(ModuleRemoved { module: entity });
            }
        }
    }
}

fn drag(mut draggings: Query<&mut Transform, With<Dragging>>, cursor: Res<CursorLocation>) {
    if let Some(position) = cursor.world_position() {
        for mut transform in draggings.iter_mut() {
            transform.translation = position.extend(2.0);
        }
    }
}
