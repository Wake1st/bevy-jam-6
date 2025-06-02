use bevy::prelude::*;

pub mod drag;
pub mod drop;

/// Made using the notes from 'umutkarakoc': <https://www.reddit.com/r/bevy/comments/16zrd7s/ui_and_inventory_drag_and_drop/?rdt=52677>
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((drag::plugin, drop::plugin));
}
