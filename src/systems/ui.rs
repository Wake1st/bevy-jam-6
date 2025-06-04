use std::fmt::Write;

use bevy::{prelude::*, text::TextSpanAccess};
use bevy_cursor::CursorLocation;

use crate::{
    theme::palette::{ENERGY_COLOR, UI_BACKGROUND},
    types::module::{BASE_BOUNCE, BASE_LENGTH, BASE_RADIUS, BASE_STRENGTH, ModuleVarient},
};

use super::{
    currency::{Currency, CurrencyAdjusted, PRICE_GENERATOR, PRICE_GONG, PRICE_LAZER, PRICE_TESLA},
    shop::Purchased,
};

const UI_BASE: Vec2 = Vec2::new(1400., 106.);
const UI_LAYER: f32 = 0.9;
const UI_BOTTOM: f32 = -363.;
const UI_X_OFFSET: f32 = 460.;
const UI_GAP: f32 = 16.;
const UI_LENGTH: f32 = 64.;

const FONT_SIZE: f32 = 44.;
const COUNTER_SIZE: Vec2 = Vec2::new(160., 60.);
const COUNTER_X_OFFSET_LABEL: f32 = -392.;
const COUNTER_X_OFFSET_NUM: f32 = 38.;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_shop)
            .add_systems(Update, (check_button_selected, update_currency_text));
    }
}

#[derive(Component, Debug)]
pub struct ShopButton(pub ModuleVarient);

#[derive(Component, Debug)]
pub struct CurrencyText;

fn spawn_shop(mut commands: Commands, asset_server: Res<AssetServer>) {
    let base = commands
        .spawn((
            Name::new("Shop Background"),
            Sprite::from_color(UI_BACKGROUND, UI_BASE),
            Transform::from_xyz(0., UI_BOTTOM, UI_LAYER),
            children![
                (
                    Name::new("Currency Label"),
                    Text2d::new("Currency: "),
                    TextFont {
                        font: asset_server.load("fonts/SixtyfourConvergence-Regular.ttf"),
                        font_size: FONT_SIZE,
                        ..default()
                    },
                    TextColor(ENERGY_COLOR),
                    Transform::from_xyz(COUNTER_X_OFFSET_LABEL, 0., 0.),
                ),
                (
                    Name::new("Currency Number"),
                    CurrencyText,
                    Text2d::new("0"),
                    TextFont {
                        font: asset_server.load("fonts/SixtyfourConvergence-Regular.ttf"),
                        font_size: FONT_SIZE,
                        ..default()
                    },
                    TextColor(ENERGY_COLOR),
                    Transform::from_xyz(COUNTER_X_OFFSET_NUM, 0., 0.),
                )
            ],
        ))
        .id();

    commands
        .spawn((
            spawn_shop_button(
                Vec2::new(UI_X_OFFSET + (UI_GAP + UI_LENGTH) * -1.5, -8.),
                asset_server.load("images/gong.png"),
                ModuleVarient::Gong(BASE_STRENGTH),
                asset_server.load("fonts/SixtyfourConvergence-Regular.ttf"),
                PRICE_GONG,
            ),
            ChildOf(base),
        ))
        .observe(shop_button_selected);
    commands
        .spawn((
            spawn_shop_button(
                Vec2::new(UI_X_OFFSET + (UI_GAP + UI_LENGTH) * -0.5, -8.),
                asset_server.load("images/generator.png"),
                ModuleVarient::Generator(BASE_RADIUS),
                asset_server.load("fonts/SixtyfourConvergence-Regular.ttf"),
                PRICE_GENERATOR,
            ),
            ChildOf(base),
        ))
        .observe(shop_button_selected);
    commands
        .spawn((
            spawn_shop_button(
                Vec2::new(UI_X_OFFSET + (UI_GAP + UI_LENGTH) * 0.5, -8.),
                asset_server.load("images/tesla.png"),
                ModuleVarient::Tesla(BASE_BOUNCE),
                asset_server.load("fonts/SixtyfourConvergence-Regular.ttf"),
                PRICE_TESLA,
            ),
            ChildOf(base),
        ))
        .observe(shop_button_selected);
    commands
        .spawn((
            spawn_shop_button(
                Vec2::new(UI_X_OFFSET + (UI_GAP + UI_LENGTH) * 1.5, -8.),
                asset_server.load("images/lazer.png"),
                ModuleVarient::Lazer(BASE_LENGTH),
                asset_server.load("fonts/SixtyfourConvergence-Regular.ttf"),
                PRICE_LAZER,
            ),
            ChildOf(base),
        ))
        .observe(shop_button_selected);
}

fn spawn_shop_button(
    position: Vec2,
    texture: Handle<Image>,
    varient: ModuleVarient,
    font: Handle<Font>,
    price: i128,
) -> impl Bundle {
    (
        Name::new("Shop Button"),
        Sprite {
            image: texture,
            custom_size: Some(Vec2::splat(UI_LENGTH)),
            ..default()
        },
        ShopButton(varient),
        Transform::from_translation(position.extend(0.)),
        children![(
            Name::new("Price"),
            Text2d::new(format!("{:?}", price)),
            TextFont {
                font,
                font_size: 16.0,
                ..default()
            },
            TextColor(ENERGY_COLOR),
            Transform::from_xyz(0., 44., 0.),
        )],
    )
}

fn update_currency_text(
    mut query: Query<&mut Text2d, With<CurrencyText>>,
    currency: Res<Currency>,
) {
    for mut text in &mut query {
        *text = Text2d::new(format!("{}", currency.0));
    }
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
