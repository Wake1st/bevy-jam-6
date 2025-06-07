use bevy::{audio::Volume, prelude::*};

const LOCAL_VOLUME: f32 = 0.4;
pub const GLOBAL_VOLUME: f32 = 0.2;
// pub const HUB_SFX_PATH: &str = "sounds/hub_ping.ogg";
pub const WAVE_SFX_PATH: &str = "sounds/wave.ogg";

pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<QueueSFX>();
        app.add_systems(Update, play_sound_once);
    }
}

#[derive(Event, Debug)]
pub struct QueueSFX {
    // pub sfx: Sfx,
    pub entity: Entity,
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum Sfx {
//     Hub,
//     Wave,
//     // FIELD,
//     // BOLT,
//     // BEAM,
// }

fn play_sound_once(mut reader: EventReader<QueueSFX>, mut sfx_sinks: Query<&mut AudioSink>) {
    for e in reader.read() {
        let Ok(mut sink) = sfx_sinks.get_mut(e.entity) else {
            continue;
        };

        sink.set_volume(Volume::Linear(LOCAL_VOLUME));
        sink.play();
    }
}
