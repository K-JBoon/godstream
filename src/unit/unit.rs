use crate::*;

#[derive(Component, Debug, Clone, Reflect, serde::Deserialize)]
pub struct Unit {
    pub pantheon: Pantheon,
    pub domains: Vec<Domain>,
    pub name: String,
    pub description: String,
    pub spritesheet: String,
}

#[derive(Debug, Clone, Reflect, serde::Deserialize, Asset)]
pub struct UnitCollection(HashMap<String, Unit>);

#[derive(Resource)]
#[allow(dead_code)]
struct LoadedFolders {
    units: Handle<bevy::asset::LoadedFolder>,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn get_unit_by_name(name: String, units_collections: Res<Assets<UnitCollection>>) -> Option<Unit> {
    if let Some((_, b)) = units_collections.iter().find(|(_, collection)| {
        collection.0.contains_key(&name)
    }) {
        b.0.get(&name).cloned()
    } else {
        None
    }
}

pub fn load_units(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(LoadedFolders {
        units: asset_server.load_folder("data/units/")
    });
}

pub fn spawn_unit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    hovered_cell_position: Res<HoveredCellPosition>,
    unit_collections: Res<Assets<UnitCollection>>,
    ally_tiles: Query<
        (Entity, &Transform, &CellPosition),
        (
            With<BattlefieldMarker>,
            With<AllyMarker>,
            Without<OccupiedMarker>,
        ),
    >,
) {
    if let Some(unit) = get_unit_by_name(String::from("Koala"), unit_collections) {
            println!("{:?}", unit);
        for (tile, transform, cell_position) in ally_tiles.iter() {
            if cell_position.0.x == hovered_cell_position.0 .0.x
                && cell_position.0.y == hovered_cell_position.0 .0.y
            {
                let mut unit_transform = *transform;
                unit_transform.rotate_axis(Dir3::Y, std::f32::consts::PI);
                unit_transform.translation.y += (TILE_SIZE as f32) / 2.0;

                commands.spawn((
                    AllyMarker,
                    UnitMarker,
                    CellPosition(cell_position.0),
                    Name::new(format!("Ally_{}", unit.name.clone())),
                    unit.clone(),
                ));

                // Mark the tile as occupied
                commands.entity(tile).insert(OccupiedMarker);
            }
        }
    }
}
