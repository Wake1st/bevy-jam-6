use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    core_pipeline::{
        bloom::{Bloom, BloomPrefilter},
        tonemapping::{DebandDither, Tonemapping},
    },
    prelude::*,
};
use bevy_cursor::TrackCursorPlugin;

use crate::{
    game::GamePlugin,
    systems::{SystemsPlugin, audio::GLOBAL_VOLUME},
    theme::palette::BACKGROUND_COLOR,
};

#[cfg(feature = "dev")]
mod dev_tools;
mod dnd;
mod game;
mod systems;
mod theme;
mod types;

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
        ))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins((
            GamePlugin,
            SystemsPlugin,
            dnd::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
        ))
        .configure_sets(
            Update,
            (
                ScheduleSystems::TickTimers,
                ScheduleSystems::RecordInput,
                ScheduleSystems::Update,
                ScheduleSystems::AudioFeedback,
                ScheduleSystems::Despawn,
            )
                .chain(),
        )
        .add_systems(Startup, setup)
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

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum ScheduleSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
    AudioFeedback,
    Despawn,
}
