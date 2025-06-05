use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    core_pipeline::{
        bloom::{Bloom, BloomPrefilter},
        tonemapping::{DebandDither, Tonemapping},
    },
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
use bevy_cursor::TrackCursorPlugin;
use bevy_egui::{EguiContext, EguiContextPass, EguiPlugin, egui};
use bevy_inspector_egui::bevy_inspector;
use systems::SystemsPlugin;

use crate::systems::audio::GLOBAL_VOLUME;

mod dnd;
mod systems;
mod theme;
mod types;

const BACKGROUND_COLOR: Color =
    Color::srgb(0.1843137254901961, 0.12549019607843137, 0.2549019607843137);

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Chain Reaction".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::Linear(GLOBAL_VOLUME),
                    },
                    ..default()
                }),
            TrackCursorPlugin,
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            bevy_inspector_egui::DefaultInspectorConfigPlugin,
        ))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins((SystemsPlugin, dnd::plugin))
        .configure_sets(
            Update,
            (
                AppSet::TickTimers,
                AppSet::RecordInput,
                AppSet::Update,
                AppSet::AudioFeedback,
                AppSet::Despawn,
            )
                .chain(),
        )
        .add_systems(Startup, setup)
        .add_systems(EguiContextPass, inspector_ui)
        .run()
}

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    // starting node
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..default()
        },
        Tonemapping::AgX,
        Bloom {
            intensity: 0.4,
            low_frequency_boost: 0.6,
            low_frequency_boost_curvature: 1.0,
            high_pass_frequency: 0.2,
            prefilter: BloomPrefilter {
                threshold: 0.2,
                threshold_softness: 0.1,
            },
            max_mip_dimension: 420,
            ..default()
        },
        DebandDither::Enabled,
        MainCamera,
        Transform::from_xyz(0., -56., 0.),
    ));
}

fn inspector_ui(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .expect("EguiContext not found")
        .clone();

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            bevy_inspector::ui_for_world(world, ui);

            // works with any `Reflect` value, including `Handle`s
            let mut any_reflect_value: i32 = 5;
            bevy_inspector::ui_for_value(&mut any_reflect_value, ui, world);

            egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            });

            ui.heading("Entities");
            bevy_inspector::ui_for_entities(world, ui);
        });
    });
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
    AudioFeedback,
    Despawn,
}
