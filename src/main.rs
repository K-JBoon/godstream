use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::fmt;

// Core Bevy imports
pub(crate) use bevy::{input::common_conditions::*, prelude::*, window::WindowMode};

// Input
use bevy_ineffable::prelude::*;

// Debugging
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// Data
use bevy_common_assets::ron::RonAssetPlugin;

// Game Modules
mod common;
use common::*;
mod ui;
use ui::*;
mod audio;
use audio::*;
mod states;
use states::*;
mod world;
use world::*;
mod unit;
use unit::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen,
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        IneffablePlugin,
    ))
    .add_systems(Startup, setup);

    // DEBUG UI
    app.add_plugins(WorldInspectorPlugin::new());

    // Game Plugins
    app.add_plugins((
        common_plugin,
        ui_plugin,
        audio_plugin,
        unit_plugin,
        stage_plugin,
    ));

    // Game State
    app.init_state::<AppState>();
    app.init_state::<GameState>();

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
