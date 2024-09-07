use super::*;

const BATTLEFIELD_WIDTH: u32 = 16;
const BATTLEFIELD_HEIGHT: u32 = 8;

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
            let tilesheet = Tilesheet {
                asset_path: String::from("spritesheets/maps/hell.png"),
                tile_width: 48.0,
                tile_height: 48.0,
                columns: 16,
                rows: 16,
            };

            let texture_handle: Handle<Image> = asset_server.load(tilesheet.asset_path.clone());

            let map_size = TilemapSize { x: 32, y: 32 };
            let tilemap_entity = commands.spawn_empty().id();
            let mut tile_storage = TileStorage::empty(map_size);

            let margin_width = (map_size.x - BATTLEFIELD_WIDTH) / 2;
            let margin_height = (map_size.y - (BATTLEFIELD_HEIGHT * 2)) / 2;

            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    let tile_pos = TilePos { x, y };
                    let tile_entity = commands
                        .spawn((
                            StageMarker,
                            TileBundle {
                                position: tile_pos,
                                tilemap_id: TilemapId(tilemap_entity),
                                ..Default::default()
                            },
                        ))
                        .id();
                    tile_storage.set(&tile_pos, tile_entity);

                    if (x < margin_width || x >= map_size.x - margin_width)
                        || (y < margin_height || y >= map_size.y - margin_height)
                    {
                        commands
                            .entity(tile_entity)
                            .insert(TileTextureIndex(tilesheet.position_to_index(14, 1)));
                    } else {
                        // Spawn center battlefield
                        let cell_x = x - margin_width;
                        let cell_y = margin_height.abs_diff(y);
                        let is_enemy = cell_y < BATTLEFIELD_HEIGHT;

                        // Generic components shared by all tiles on the battlefield
                        // tile_entity.insert((StageMarker, BattlefieldMarker));

                        // Add the tile specific components,
                        // for position some things need to be inverted
                        // to have the expected coordinates on both sides,
                        // see CellPosition
                        if is_enemy {
                            commands.entity(tile_entity).insert((
                                TileTextureIndex(tilesheet.position_to_index(5, 0)),
                                EnemyMarker,
                                CellPosition(Vec2::new(
                                    (BATTLEFIELD_WIDTH - 1 - cell_x) as f32,
                                    cell_y as f32,
                                )),
                            ));
                        } else {
                            commands.entity(tile_entity).insert((
                                TileTextureIndex(tilesheet.position_to_index(8, 2)),
                                AllyMarker,
                                CellPosition(Vec2::new(
                                    cell_x as f32,
                                    (BATTLEFIELD_HEIGHT - 1) as f32
                                        - (cell_y - BATTLEFIELD_HEIGHT) as f32,
                                )),
                            ));
                        }
                    }
                }
            }

            let tile_size = TilemapTileSize {
                x: tilesheet.tile_width,
                y: tilesheet.tile_height,
            };
            let grid_size = tile_size.into();
            let map_type = TilemapType::default();

            commands.entity(tilemap_entity).insert(TilemapBundle {
                grid_size,
                map_type,
                size: map_size,
                storage: tile_storage,
                texture: TilemapTexture::Single(texture_handle),
                tile_size,
                transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
                ..Default::default()
            });
        }
    }

    next_app_state.set(AppState::InGame);
}
