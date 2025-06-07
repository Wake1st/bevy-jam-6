use bevy::prelude::*;
use bevy_cursor::CursorLocation;

use crate::types::module::{
    ModuleVarient, // spawn_generator, spawn_lazer, spawn_tesla
    spawn_gong,
    spawn_lazer,
};

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Purchased>();
        app.add_systems(Update, purchase);
    }
}

#[derive(Event, Debug)]
pub struct Purchased {
    pub varient: ModuleVarient,
}

fn purchase(
    mut purchased: EventReader<Purchased>,
    cursor: Res<CursorLocation>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for e in purchased.read() {
        let Some(position) = cursor.world_position() else {
            return;
        };

        match e.varient {
            ModuleVarient::Gong(_) => {
                commands.spawn(spawn_gong(position, asset_server.load("images/gong.png")));
            } // ModuleVarient::Generator(_) => {
            //     commands.spawn(spawn_generator(
            //         position,
            //         asset_server.load("images/generator.png"),
            //     ));
            // }
            // ModuleVarient::Tesla(_) => {
            //     commands.spawn(spawn_tesla(position, asset_server.load("images/tesla.png")));
            // }
            ModuleVarient::Lazer(_) => {
                commands.spawn(spawn_lazer(position, asset_server.load("images/lazer.png")));
            }
        }

        info!("spawned at: {:?}", position);
    }
}
