use super::*;

#[derive(Debug, Clone, Reflect, serde::Deserialize)]
pub struct LightEmitterSettings {
    pub color: Color,
    pub intensity: f32,
    pub radius: f32,
    pub falloff: f32,
    pub cast_shadows: bool,
}

#[derive(Component, Debug, Clone, Reflect, serde::Deserialize)]
pub struct Tilesheet {
    pub spritesheet: Spritesheet,
    pub lights: HashMap<SpritesheetCellIdentifier, LightEmitterSettings>,
    pub occluders: Vec<SpritesheetCellIdentifier>,
}

#[derive(Debug, Clone, Reflect, serde::Deserialize, Asset)]
pub struct TilesheetCollection(HashMap<String, Tilesheet>);

#[derive(AssetCollection, Resource)]
pub struct TilesheetAssets {
    #[asset(path = "data/tilesheets/", collection)]
    _folder: Vec<UntypedHandle>,
}

impl Tilesheet {
    pub fn cell_to_index(&self, cell: &SpritesheetCell) -> Option<u32> {
        match cell {
            SpritesheetCell::Static(identifier) => self.identifier_to_index(identifier),
            SpritesheetCell::Animated(identifiers) => {
                if !identifiers.is_empty() {
                    self.identifier_to_index(identifiers.first().unwrap())
                } else {
                    None
                }
            }
        }
    }

    pub fn identifier_to_index(&self, identifier: &SpritesheetCellIdentifier) -> Option<u32> {
        match identifier {
            SpritesheetCellIdentifier::Name(name) => self.name_to_index(name),
            SpritesheetCellIdentifier::Position(position) => {
                self.position_to_index(position.0, position.1)
            }
            SpritesheetCellIdentifier::None => None,
        }
    }

    pub fn name_to_index(&self, name: &String) -> Option<u32> {
        if let Some(cell) = self.spritesheet.cell_names.get(name) {
            self.cell_to_index(cell)
        } else {
            warn!(
                "Tried to access named cell '{}' which is not defined for Spritesheet '{}'",
                name, self.spritesheet.asset_path
            );
            None
        }
    }

    pub fn position_to_index(&self, row: u32, column: u32) -> Option<u32> {
        if row > self.spritesheet.rows - 1 || column > self.spritesheet.columns - 1 {
            warn!(
                "Incorrect position request ({}, {}) for ({}, {}, {})",
                row,
                column,
                self.spritesheet.asset_path,
                self.spritesheet.rows,
                self.spritesheet.columns
            );
            None
        } else {
            Some(row * self.spritesheet.columns + column)
        }
    }
}

pub fn get_tilesheet_by_name(
    name: String,
    tilesheet_collections: &Res<Assets<TilesheetCollection>>,
) -> Option<Tilesheet> {
    if let Some((_, b)) = tilesheet_collections
        .iter()
        .find(|(_, collection)| collection.0.contains_key(&name))
    {
        b.0.get(&name).cloned()
    } else {
        None
    }
}
