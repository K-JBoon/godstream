use crate::*;

const BATTLEFIELD_WIDTH: u32 = 8;
const BATTLEFIELD_HEIGHT: u32 = 4;

pub enum Stage {
    ArenaBase,
}

pub fn spawn_stage(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_stage: Res<CurrentStage>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    match current_stage.stage {
        Stage::ArenaBase => {
            let stage_width: u32 = 12;
            let stage_height: u32 = 12;
            let margin_width = (stage_width - BATTLEFIELD_WIDTH) / 2;
            let margin_height = (stage_height - (BATTLEFIELD_HEIGHT * 2)) / 2;

            for x in 0..stage_width {
                for z in 0..stage_height {
                    if (x < margin_width || x >= stage_width - margin_width)
                        || (z < margin_height || z >= stage_height - margin_height)
                    {
                        commands.spawn((
                            StageMarker,
                            SceneBundle {
                                scene: asset_server.load(Tile::BlockGrass.to_string()),
                                transform: Transform::from_xyz(
                                    (x * TILE_SIZE) as f32,
                                    0.0,
                                    (z * TILE_SIZE) as f32,
                                ),
                                ..default()
                            },
                        ));
                    } else {
                        // Spawn center battlefield
                        let cell_x = x - margin_width;
                        let cell_z = margin_height.abs_diff(z);
                        let is_enemy = cell_z < BATTLEFIELD_HEIGHT;

                        // Generic components shared by all tiles on the battlefield
                        let mut entity = commands.spawn((
                            StageMarker,
                            BattlefieldMarker,
                        ));

                        // Add the tile specific components,
                        // for position some things need to be inverted
                        // to have the expected coordinates on both sides,
                        // see CellPosition
                        if is_enemy {
                            entity.insert((
                                EnemyMarker,
                                CellPosition(Vec2::new(
                                    (BATTLEFIELD_WIDTH - 1 - cell_x) as f32,
                                    cell_z as f32)
                                ),
                                SceneBundle {
                                    scene: asset_server
                                        .load(Tile::BattlefieldBlockEnemyStone.to_string()),
                                    transform: Transform::from_xyz(
                                        (x * TILE_SIZE) as f32,
                                        0.0,
                                        (z * TILE_SIZE) as f32,
                                    ),
                                    ..default()
                                },
                            ));
                        } else {
                            entity.insert((
                                AllyMarker,
                                CellPosition(Vec2::new(
                                    cell_x as f32,
                                    (BATTLEFIELD_HEIGHT - 1) as f32 - (cell_z - BATTLEFIELD_HEIGHT) as f32)
                                ),
                                SceneBundle {
                                    scene: asset_server
                                        .load(Tile::BattlefieldBlockAllyStone.to_string()),
                                    transform: Transform::from_xyz(
                                        (x * TILE_SIZE) as f32,
                                        0.0,
                                        (z * TILE_SIZE) as f32,
                                    ),
                                    ..default()
                                },
                            ));
                        }
                    }
                }
            }
        }
    }

    next_app_state.set(AppState::InGame);
}
