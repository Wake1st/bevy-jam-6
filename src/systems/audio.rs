use bevy::prelude::*;

pub const GLOBAL_VOLUME: f32 = 0.2;
pub const HUB_SFX_PATH: &'static str = "sounds/hub_ping.ogg";
pub const WAVE_SFX_PATH: &'static str = "sounds/wave.ogg";

pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<QueueSFX>();
        app.add_systems(Update, play_sound);
    }
}

#[derive(Event, Debug)]
pub struct QueueSFX {
    pub sfx: SFX,
    pub entity: Entity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SFX {
    HUB,
    WAVE,
    // FIELD,
    // BOLT,
    // BEAM,
}

fn play_sound(mut reader: EventReader<QueueSFX>, mut sfx_sinks: Query<&AudioSink>) {
    for e in reader.read() {
        let Ok(sink) = sfx_sinks.get_mut(e.entity) else {
            continue;
        };

        sink.play();
    }
}
