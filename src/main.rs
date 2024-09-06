// Core Bevy imports
pub(crate) use bevy::{
    prelude::*,
    window::WindowMode,
    input::common_conditions::*,
};

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
        RonAssetPlugin::<AllyUnit>::new(&["ally_unit.ron"]),
    ))
    .add_systems(Startup, (setup, load_ally_units))
    .insert_resource(AmbientLight {
        color: Color::srgb_u8(255, 255, 255),
        brightness: 0.75,
    });

    // Game Plugins
    app.add_plugins((audio_plugin, common_plugin));

    // DEBUG UI
    app.add_plugins(WorldInspectorPlugin::new())
        .register_type::<AllyUnit>()
        .register_type::<CellPosition>();

    // Game Resources
    app.insert_resource(CurrentStage {
        stage: Stage::ArenaBase,
    });

    app.init_state::<AppState>();
    app.init_state::<GameState>();

    // Setup core game systems
    app.add_systems(
        OnEnter(AppState::LoadingScreen),
        (despawn::<StageMarker>, spawn_stage)
    );

    app.add_systems(Update,
        (
            spawn_ally_unit.run_if(input_just_pressed(KeyCode::KeyQ)),
        )
    );

    app.run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Camera3dBundle {
                camera: Camera {
                    clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                    hdr: true,
                    ..default()
                },
                transform: Transform::from_xyz(950.0, 550.0, 950.0)
                    .looking_at(Vec3::new(150.0, -200.0, 50.0), Vec3::Y),
                ..default()
            },
        ));
}
