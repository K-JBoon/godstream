use crate::*;
use bevy_kira_audio::prelude::*;

mod background_music;
use background_music::*;

pub fn audio_plugin(app: &mut App) {
    app.add_plugins(AudioPlugin);
    app.add_systems(Startup, play_background_music);
}
