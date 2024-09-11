use std::collections::HashMap;
use std::fmt;

// Core Bevy imports
pub(crate) use bevy::{input::common_conditions::*, prelude::*, window::WindowMode};

// Loading state
use bevy_asset_loader::prelude::*;

// Debugging
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use iyes_perf_ui::prelude::*;

// Data
use bevy_common_assets::ron::RonAssetPlugin;

// Lighting
use bevy_light_2d::prelude::*;

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
mod units;
use units::*;

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
        Light2dPlugin,
    ))
    .init_state::<AppState>()
    .add_loading_state(
        LoadingState::new(AppState::AssetLoading).continue_to_state(AppState::PrepareStage),
    )
    .add_systems(Startup, setup);

    // DEBUG UI
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin);

    // Game Plugins
    app.add_plugins((
        common_plugin,
        ui_plugin,
        audio_plugin,
        unit_plugin,
        stage_plugin,
    ));

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: 1.2,
                near: -1000.0,
                far: 1000.0,
                ..default()
            },
            ..default()
        },
        AmbientLight2d {
            brightness: 0.05,
            ..default()
        },
    ));

    // create a simple Perf UI with default settings
    // and all entries provided by the crate:
    commands.spawn(PerfUiCompleteBundle::default());
}
