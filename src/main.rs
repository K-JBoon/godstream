// Core Bevy imports
pub(crate) use bevy::{input::common_conditions::*, prelude::*, window::WindowMode};

// Audio
use bevy_kira_audio::prelude::*;

// Input
use bevy_ineffable::prelude::*;

// Debugging
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// Voxel handlers
use bevy_vox_scene::VoxScenePlugin;

// Data
use bevy_common_assets::ron::RonAssetPlugin;

// Game Modules
mod common;
use common::*;
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
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen,
                resizable: false,
                ..default()
            }),
            ..default()
        }),
        IneffablePlugin,
        VoxScenePlugin::default(),
    ))
    .add_systems(Startup, setup)
    .insert_resource(AmbientLight {
        color: Color::srgb_u8(255, 255, 255),
        brightness: 0.75,
    });

    // DEBUG UI
    app.add_plugins(WorldInspectorPlugin::new());

    // Game Plugins
    app.add_plugins((common_plugin, audio_plugin, unit_plugin, stage_plugin));

    // Game State
    app.init_state::<AppState>();
    app.init_state::<GameState>();

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
            hdr: true,
            ..default()
        },
        transform: Transform::from_xyz(950.0, 550.0, 950.0)
            .looking_at(Vec3::new(150.0, -200.0, 50.0), Vec3::Y),
        ..default()
    },));
}
