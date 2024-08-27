// Core Bevy imports
pub(crate) use bevy::{
    prelude::*,
    render::render_resource::*,
    window::WindowMode,
    sprite::Anchor,
};
pub(crate) use bevy_lunex::prelude::*;

// Voxel handlers
use bevy_vox_scene::{VoxScenePlugin, VoxelSceneBundle};

// Game Modules
mod states;
use states::*;
mod ui;
use {ui::constants::*, ui::components::*, ui::routes::*};
mod world;
use world::*;

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
        VoxScenePlugin,
        UiPlugin,
    ))
    .add_plugins(ComponentPlugin)
    .add_plugins(RoutePlugin)
    .add_systems(Startup, setup)
    .insert_resource(CurrentStage { stage: Stage::ArenaGrass });

    app.init_state::<AppState>();
    app.init_state::<GameState>();

    // Setup core game systems
    app.add_systems(OnEnter(AppState::LoadingScreen), (
        despawn_stage,
        spawn_stage,
    ));
    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Set up the main camera, the UI will render into this,
    // and the game world will be rendered into a texture that's
    // rendered into this camera
    commands.spawn((
        MainUi,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            ..default()
        },
    ));

    // Create a texture resource that our 3D camera will render to
    let size = Extent3d {
        width: 1920,
        height: 1080,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // Initiate the image
    image.resize(size);

    // Add our texture to asset server and get a handle
    let render_image = asset_server.add(image);
    commands
        .spawn((
            Camera3dBundle {
                camera: Camera {
                    order: -1,
                    target: render_image.clone().into(),
                    clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                    hdr: true,
                    ..default()
                },
                transform: Transform::from_xyz(200.0, 200.0, 200.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
        ));

    commands
        .spawn((
            MovableByCamera,
            UiTreeBundle::<MainUi>::from(UiTree::new2d("Hello UI!")),
        ))
        .with_children(|ui| {
            ui.spawn((
                // Link the entity
                UiLink::<MainUi>::path("Root"),
                // Specify UI layout
                UiLayout::window_full()
                    .pos(Ab(20.0))
                    .size(Rl(100.0) - Ab(40.0))
                    .pack::<Base>(),
            ));

            ui.spawn((
                // Link the entity
                UiLink::<MainUi>::path("Root/Rectangle"),
                // Specify UI layout
                UiLayout::solid().size(Ab((1920.0, 1080.0))).pack::<Base>(),
            ));

            // Spawn 3D camera view
            ui.spawn((
                UiLink::<MainUi>::path("Root/Camera3d"),
                UiLayout::solid()
                    .size((1920.0, 1080.0))
                    .scaling(Scaling::Fill)
                    .pack::<Base>(),
                UiImage2dBundle::from(render_image),
                PickingPortal, // Send pick events through to the 3D camera
            ));
        });

    // Enable the cursor
    commands.spawn(CursorBundle::default());

    // Show our UI
    // TODO: this should be conditional on the state
    commands.spawn(MyRoute);
}
