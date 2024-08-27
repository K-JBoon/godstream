use crate::*;

const TILE_SIZE: f32 = 64.0;

pub enum Stage {
    ArenaGrass,
    ArenaWater,
}

#[derive(Resource)]
pub struct CurrentStage {
    pub stage: Stage
}

pub fn despawn_stage(
    mut commands: Commands,
    query: Query<Entity, With<VoxelStage>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_stage(mut commands: Commands, asset_server: Res<AssetServer>, current_stage: Res<CurrentStage>, mut next_app_state: ResMut<NextState<AppState>>) {
    match current_stage.stage {
        Stage::ArenaGrass => {
            for x in 0..2 {
                for z in 0..2 {
                    commands.spawn((VoxelStage, VoxelSceneBundle {
                        scene: asset_server.load(Tile::GrassBlock.to_string()),
                        transform: Transform::from_xyz((x as f32) * TILE_SIZE, 0.0, (z as f32) * TILE_SIZE),
                        ..default()
                    }));
                }
            }
        },
        Stage::ArenaWater => {
            for x in 0..5 {
                for z in 0..5 {
                    commands.spawn((VoxelStage, VoxelSceneBundle {
                        scene: asset_server.load(Tile::WaterGrassBlock.to_string()),
                        transform: Transform::from_xyz((x as f32) * TILE_SIZE, 0.0, (z as f32) * TILE_SIZE),
                        ..default()
                    }));
                }
            }
        },
    }

    next_app_state.set(AppState::InGame);
}
