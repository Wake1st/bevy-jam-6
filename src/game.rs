use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResetGame>();
    }
}

#[derive(Event)]
pub struct ResetGame;

pub fn reset_game(event: EventReader<ResetGame>) -> bool {
    !event.is_empty()
}
