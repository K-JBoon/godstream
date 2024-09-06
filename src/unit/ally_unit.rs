use crate::*;
use rand::seq::SliceRandom;
use std::fmt;

const UNITS: &[&str] = &[
    "data/ally_units/egyptian_anubis.ally_unit.ron",
    // "data/ally_units/egyptian_bastet.ally_unit.ron",
    // "data/ally_units/egyptian_horus.ally_unit.ron",
    // "data/ally_units/egyptian_isis.ally_unit.ron",
    // "data/ally_units/egyptian_osiris.ally_unit.ron",
    // "data/ally_units/egyptian_ra.ally_unit.ron",
    // "data/ally_units/egyptian_sekhmet.ally_unit.ron",
    // "data/ally_units/egyptian_sobek.ally_unit.ron",
    // "data/ally_units/egyptian_thoth.ally_unit.ron",
    // "data/ally_units/greek_apollo.ally_unit.ron",
    // "data/ally_units/greek_ares.ally_unit.ron",
    // "data/ally_units/greek_artemis.ally_unit.ron",
    // "data/ally_units/greek_athena.ally_unit.ron",
    // "data/ally_units/greek_demeter.ally_unit.ron",
    // "data/ally_units/greek_hades.ally_unit.ron",
    // "data/ally_units/greek_hecate.ally_unit.ron",
    // "data/ally_units/greek_poseidon.ally_unit.ron",
    // "data/ally_units/greek_zeus.ally_unit.ron",
    // "data/ally_units/norse_freyja.ally_unit.ron",
    // "data/ally_units/norse_heimdall.ally_unit.ron",
    // "data/ally_units/norse_hel.ally_unit.ron",
    // "data/ally_units/norse_loki.ally_unit.ron",
    // "data/ally_units/norse_njord.ally_unit.ron",
    // "data/ally_units/norse_odin.ally_unit.ron",
    // "data/ally_units/norse_skadi.ally_unit.ron",
    // "data/ally_units/norse_thor.ally_unit.ron",
    // "data/ally_units/norse_tyr.ally_unit.ron",
];

#[derive(Component, Debug, Clone, Reflect, serde::Deserialize, Asset)]
pub struct AllyUnit {
    pub pantheon: Pantheon,
    pub domains: Vec<Domain>,
    pub name: String,
    pub description: String,
    pub model: String,
}

#[derive(Resource)]
#[allow(dead_code)]
struct AllyUnits(Vec<Handle<AllyUnit>>);

impl fmt::Display for AllyUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn load_ally_units(mut commands: Commands, asset_server: Res<AssetServer>) {
    let units: Vec<Handle<AllyUnit>> = UNITS
        .iter()
        .map(|path| {
            let ally_unit_handle: Handle<AllyUnit> = asset_server.load(path.to_string());
            ally_unit_handle
        })
        .collect();

    commands.insert_resource(AllyUnits(units));
}

pub fn spawn_ally_unit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    hovered_cell_position: Res<HoveredCellPosition>,
    ally_units: Res<Assets<AllyUnit>>,
    ally_tiles: Query<
        (Entity, &Transform, &CellPosition),
        (
            With<BattlefieldMarker>,
            With<AllyMarker>,
            Without<OccupiedMarker>,
        ),
    >,
) {
    let ids: Vec<AssetId<AllyUnit>> = ally_units.ids().collect();
    let random_id = ids.choose(&mut rand::thread_rng()).unwrap();

    let unit = ally_units.get(*random_id).unwrap();

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
                SceneBundle {
                    scene: asset_server.load(unit.model.clone()),
                    transform: unit_transform,
                    ..default()
                },
                unit.clone(),
            ));

            // Mark the tile as occupied
            commands.entity(tile).insert(OccupiedMarker);
        }
    }
}
