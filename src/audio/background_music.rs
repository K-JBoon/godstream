use crate::*;

pub fn play_background_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("sounds/music/ambient_chimes.mp3.ron"));
}
