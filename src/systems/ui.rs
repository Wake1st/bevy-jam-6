use bevy::prelude::*;
use bevy_cursor::CursorLocation;

use crate::{
    theme::{
        palette::{ENERGY_COLOR, UI_BACKGROUND},
        widget,
    },
    types::module::{BASE_BOUNCE, BASE_COUNT, BASE_RADIUS, BASE_STRENGTH, ModuleVarient},
};

use super::{
    currency::{Currency, CurrencyAdjusted, PRICE_GENERATOR, PRICE_GONG, PRICE_LAZER, PRICE_TESLA},
    shop::Purchased,
};

const UI_BASE: Vec2 = Vec2::new(1200., 300.);
const UI_LAYER: f32 = 0.5;
const UI_BOTTOM: f32 = -280.;
const UI_GAP: f32 = 32.;
const UI_LENGTH: f32 = 128.;

const COUNTER_SIZE: Vec2 = Vec2::new(60., 160.);

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_shop)
            .add_systems(Update, check_button_selected);
    }
}

#[derive(Component, Debug)]
pub struct ShopButton(pub ModuleVarient);

fn spawn_shop(mut commands: Commands, asset_server: Res<AssetServer>) {
    let base = commands
        .spawn((
            Name::new("Shop Background"),
            Sprite::from_color(UI_BACKGROUND, UI_BASE),
        ))
        .id();

    commands
        .spawn((
            spawn_shop_button(
                Vec2::new((UI_GAP + UI_LENGTH) * -1.5, UI_BOTTOM),
                asset_server.load("images/gong.png"),
                ModuleVarient::Gong(BASE_STRENGTH),
            ),
            ChildOf(base),
        ))
        .observe(shop_button_selected);
    commands
        .spawn((
            spawn_shop_button(
                Vec2::new((UI_GAP + UI_LENGTH) * -0.5, UI_BOTTOM),
                asset_server.load("images/generator.png"),
                ModuleVarient::Generator(BASE_RADIUS),
            ),
            ChildOf(base),
        ))
        .observe(shop_button_selected);
    commands
        .spawn((
            spawn_shop_button(
                Vec2::new((UI_GAP + UI_LENGTH) * 0.5, UI_BOTTOM),
                asset_server.load("images/tesla.png"),
                ModuleVarient::Tesla(BASE_BOUNCE),
            ),
            ChildOf(base),
        ))
        .observe(shop_button_selected);
    commands
        .spawn((
            spawn_shop_button(
                Vec2::new((UI_GAP + UI_LENGTH) * 1.5, UI_BOTTOM),
                asset_server.load("images/lazer.png"),
                ModuleVarient::Lazer(BASE_COUNT),
            ),
            ChildOf(base),
        ))
        .observe(shop_button_selected);
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

fn shop_button_selected(
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

#[derive(Component, Debug)]
pub struct CurrencyText;

fn spawn_currency_counter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Currency Background"),
        Sprite::from_color(UI_BACKGROUND, COUNTER_SIZE),
        children![(
            Name::new("Currency Text"),
            CurrencyText,
            Text::new("Currency: 0"),
            TextFont {
                font: asset_server.load("fonts/SixtyfourConvergence-Regular.ttf"),
                font_size: 48.0,
                ..default()
            },
            TextColor(ENERGY_COLOR),
        )],
    ));
}

fn text_update_system(
    mut query: Query<&mut TextSpan, With<CurrencyText>>,
    currency: Res<Currency>,
) {
    for mut span in &mut query {
        **span = format!("Currency: {:?}", currency.0);
    }
}
