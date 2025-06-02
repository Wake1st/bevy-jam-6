use bevy::prelude::*;
use bevy_cursor::CursorLocation;

use crate::{
    theme::widget,
    types::module::{BASE_BOUNCE, BASE_COUNT, BASE_RADIUS, BASE_STRENGTH, ModuleVarient},
};

use super::{
    currency::{Currency, CurrencyAdjusted, PRICE_GENERATOR, PRICE_GONG, PRICE_LAZER, PRICE_TESLA},
    shop::Purchased,
};

const UI_LAYER: f32 = 0.5;
const UI_BOTTOM: f32 = -280.;
const UI_GAP: f32 = 32.;
const UI_LENGTH: f32 = 128.;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_menu)
            .add_systems(Update, check_button_selected);
    }
}

#[derive(Component, Debug)]
pub struct ShopButton(pub ModuleVarient);

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(spawn_shop_button(
            Vec2::new((UI_GAP + UI_LENGTH) * -1.5, UI_BOTTOM),
            asset_server.load("gong.png"),
            ModuleVarient::Gong(BASE_STRENGTH),
        ))
        .observe(make_purchase);
    commands
        .spawn(spawn_shop_button(
            Vec2::new((UI_GAP + UI_LENGTH) * -0.5, UI_BOTTOM),
            asset_server.load("generator.png"),
            ModuleVarient::Generator(BASE_RADIUS),
        ))
        .observe(make_purchase);
    commands
        .spawn(spawn_shop_button(
            Vec2::new((UI_GAP + UI_LENGTH) * 0.5, UI_BOTTOM),
            asset_server.load("tesla.png"),
            ModuleVarient::Tesla(BASE_BOUNCE),
        ))
        .observe(make_purchase);
    commands
        .spawn(spawn_shop_button(
            Vec2::new((UI_GAP + UI_LENGTH) * 1.5, UI_BOTTOM),
            asset_server.load("lazer.png"),
            ModuleVarient::Lazer(BASE_COUNT),
        ))
        .observe(make_purchase);
}

fn spawn_shop_button(
    position: Vec2,
    texture: Handle<Image>,
    varient: ModuleVarient,
) -> impl Bundle {
    (
        Name::new("Shop Button"),
        Sprite {
            image: texture,
            custom_size: Some(Vec2::splat(UI_LENGTH)),
            ..default()
        },
        ShopButton(varient),
        Transform::from_translation(position.extend(UI_LAYER)),
    )
}

#[derive(Clone, Component, Event)]
#[event(traversal = &'static ChildOf, auto_propagate)]
pub struct ShopButtonSelected;

fn check_button_selected(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    cursor: Res<CursorLocation>,
    buttons: Query<(Entity, &GlobalTransform), With<ShopButton>>,
    mut commands: Commands,
) {
    // only run when left mouse selected
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    // ensure we have a cursor
    let Some(position) = cursor.world_position() else {
        return;
    };

    // trigger selected button
    for (entity, transform) in buttons.iter() {
        let rect = Rect::from_center_size(transform.translation().xy(), Vec2::splat(UI_LENGTH));
        if rect.contains(position) {
            commands.trigger_targets(ShopButtonSelected, entity);
        }
    }
}

fn make_purchase(
    trigger: Trigger<ShopButtonSelected>,
    buttons: Query<&ShopButton>,
    currency: Res<Currency>,
    mut purchased: EventWriter<Purchased>,
    mut adjusted: EventWriter<CurrencyAdjusted>,
) {
    let Ok(button) = buttons.get(trigger.target()) else {
        return;
    };

    let cost = match button.0 {
        ModuleVarient::Gong(_) => PRICE_GONG,
        ModuleVarient::Generator(_) => PRICE_GENERATOR,
        ModuleVarient::Tesla(_) => PRICE_TESLA,
        ModuleVarient::Lazer(_) => PRICE_LAZER,
    };

    if currency.0 > cost {
        purchased.write(Purchased {
            varient: button.0.clone(),
        });
        adjusted.write(CurrencyAdjusted { amount: -cost });
    }
}
