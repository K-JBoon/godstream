use super::*;

static TILE_ANIMATION_FRAME_DURATION: u64 = 250;

#[derive(Component, Debug, Clone, Reflect, serde::Deserialize)]
pub enum LayerType {
    Background,
    AllyStage,
    EnemyStage,
    Object, // Things that collide, units, terrain, etc.
    Foreground,
}

// Structure for a single layer in the stage
#[derive(Component, Debug, Clone, Reflect, serde::Deserialize)]
pub struct StageLayer {
    pub tilesheet_name: String,
    pub layer_type: LayerType,
    pub width: u32,
    pub height: u32,
    pub x_offset: f32,
    pub y_offset: f32,
    pub cells: Vec<Vec<SpritesheetCellIdentifier>>,
}

// Structure for the entire stage
#[derive(Component, Debug, Clone, Reflect, serde::Deserialize)]
pub struct Stage {
    pub name: String,
    pub layers: Vec<StageLayer>,
}

#[derive(Debug, Clone, Reflect, serde::Deserialize, Asset)]
pub struct StageCollection(HashMap<String, Stage>);

#[derive(AssetCollection, Resource)]
pub struct StageAssets {
    #[asset(path = "data/stages/", collection)]
    _folder: Vec<UntypedHandle>,
}

#[derive(Component)]
pub struct AnimatedTile {
    pub tilesheet: Tilesheet, // TODO: we shouldn't have to copy this for every tile
    pub cell_identifiers: Vec<SpritesheetCellIdentifier>,
    pub index: usize,
}

#[derive(Resource)]
pub struct TileAnimationConfig {
    timer: Timer,
}

// Implementation for Stage
impl Stage {
    fn spawn(
        &self,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        tilesheet_collections: Res<Assets<TilesheetCollection>>,
    ) {
        self.layers
            .iter()
            .enumerate()
            .for_each(|(layer_index, layer)| {
                if let Some(tilesheet) =
                    get_tilesheet_by_name(layer.tilesheet_name.clone(), &tilesheet_collections)
                {
                    let texture_handle: Handle<Image> =
                        asset_server.load(tilesheet.spritesheet.asset_path.clone());

                    let map_size = TilemapSize {
                        x: layer.width,
                        y: layer.height,
                    };
                    let tilemap_entity = commands.spawn_empty().id();
                    let mut tile_storage = TileStorage::empty(map_size);

                    layer.cells.iter().enumerate().for_each(|(y, row)| {
                        row.iter().enumerate().for_each(|(x, cell_identifier)| {
                            // Make sure we don't go out of bounds if the data has been
                            // poorly defined
                            if (x as u32) < layer.width && (y as u32) < layer.height {
                                // Cell may be identified as None, or the index may be invalid,
                                // in which case we skip making an entity
                                if let Some(texture_index) =
                                    tilesheet.identifier_to_index(cell_identifier)
                                {
                                    let tile_pos = TilePos {
                                        x: x as u32,
                                        y: y as u32,
                                    };
                                    let tile_entity = commands
                                        .spawn(TileBundle {
                                            position: tile_pos,
                                            tilemap_id: TilemapId(tilemap_entity),
                                            ..Default::default()
                                        })
                                        .id();
                                    tile_storage.set(&tile_pos, tile_entity);

                                    commands
                                        .entity(tile_entity)
                                        .insert(TileTextureIndex(texture_index));

                                    if let SpritesheetCellIdentifier::Name(cell_name) =
                                        cell_identifier
                                    {
                                        if let Some(SpritesheetCell::Animated(cell_identifiers)) =
                                            tilesheet.spritesheet.cell_names.get(cell_name)
                                        {
                                            commands.entity(tile_entity).insert(AnimatedTile {
                                                tilesheet: tilesheet.clone(),
                                                cell_identifiers: cell_identifiers.clone(),
                                                index: 0,
                                            });
                                        }
                                    }
                                }
                            }
                        });
                    });

                    match layer.layer_type {
                        LayerType::AllyStage => {
                            commands.entity(tilemap_entity).insert(AllyMarker);
                        }
                        LayerType::EnemyStage => {
                            commands.entity(tilemap_entity).insert(EnemyMarker);
                        }
                        _ => (),
                    };

                    let tile_size = TilemapTileSize {
                        x: tilesheet.spritesheet.tile_width,
                        y: tilesheet.spritesheet.tile_height,
                    };
                    let grid_size = tile_size.into();
                    let map_type = TilemapType::default();

                    commands.entity(tilemap_entity).insert((
                        Name::new("TileBundle"),
                        StageMarker,
                        TilemapBundle {
                            grid_size,
                            map_type,
                            size: map_size,
                            storage: tile_storage,
                            texture: TilemapTexture::Single(texture_handle),
                            tile_size,
                            transform: get_tilemap_center_transform(
                                &map_size,
                                &grid_size,
                                &map_type,
                                layer_index as f32,
                            ) * Transform::from_xyz(
                                layer.x_offset * tilesheet.spritesheet.tile_width,
                                layer.y_offset * tilesheet.spritesheet.tile_height,
                                1.0,
                            ),
                            ..Default::default()
                        },
                    ));
                }
            });
    }
}

pub fn get_stage_by_name(
    name: String,
    stage_collections: &Res<Assets<StageCollection>>,
) -> Option<Stage> {
    if let Some((_, b)) = stage_collections
        .iter()
        .find(|(_, collection)| collection.0.contains_key(&name))
    {
        b.0.get(&name).cloned()
    } else {
        None
    }
}

pub fn spawn_stage(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    stage_collections: Res<Assets<StageCollection>>,
    tilesheet_collections: Res<Assets<TilesheetCollection>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Some(stage) = get_stage_by_name(String::from("Hell"), &stage_collections) {
        stage.spawn(commands, asset_server, tilesheet_collections);
    }

    next_app_state.set(AppState::OnStage);
}

pub fn setup_tile_animation(mut commands: Commands) {
    commands.insert_resource(TileAnimationConfig {
        // create the repeating timer
        timer: Timer::new(
            std::time::Duration::from_millis(TILE_ANIMATION_FRAME_DURATION),
            TimerMode::Repeating,
        ),
    });
}

pub fn animate_tiles(
    time: Res<Time>,
    mut config: ResMut<TileAnimationConfig>,
    mut tile_query: Query<(&mut TileTextureIndex, &mut AnimatedTile)>,
) {
    config.timer.tick(time.delta());

    if config.timer.finished() {
        for (mut texture_index, mut animated_tile) in tile_query.iter_mut() {
            if animated_tile.index == animated_tile.cell_identifiers.len() - 1 {
                animated_tile.index = 0;
            }

            animated_tile.index += 1;

            if let Some(next_texture_index) = animated_tile.tilesheet.identifier_to_index(
                animated_tile
                    .cell_identifiers
                    .get(animated_tile.index)
                    .unwrap(),
            ) {
                texture_index.0 = next_texture_index;
            }
        }
    }
}
