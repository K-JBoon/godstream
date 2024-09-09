use super::*;

#[derive(Component, Debug, Clone, Reflect, serde::Deserialize)]
pub struct Tilesheet {
    pub spritesheet: Spritesheet,
}

#[derive(Debug, Clone, Reflect, serde::Deserialize, Asset)]
pub struct TilesheetCollection(HashMap<String, Tilesheet>);

#[derive(AssetCollection, Resource)]
pub struct TilesheetAssets {
    #[asset(path = "data/tilesheets/", collection)]
    _folder: Vec<UntypedHandle>,
}

impl Tilesheet {
    pub fn name_to_index(&self, name: &String) -> u32 {
        if let Some(cell) = self.spritesheet.cell_names.get(name) {
            match cell {
                SpritesheetCell::Static(identifier) => match identifier {
                    SpritesheetCellIdentifier::Name(name) => self.name_to_index(name),
                    SpritesheetCellIdentifier::Position(position) => {
                        self.position_to_index(position.0, position.1)
                    }
                },
                SpritesheetCell::Animated(identifiers) => {
                    warn!("Nesting animated cells is not supported! Only the first frame will be returned!");
                    if !identifiers.is_empty() {
                        match identifiers.first().unwrap() {
                            SpritesheetCellIdentifier::Name(name) => self.name_to_index(name),
                            SpritesheetCellIdentifier::Position(position) => {
                                self.position_to_index(position.0, position.1)
                            }
                        }
                    } else {
                        0
                    }
                },
            }
        } else {
            warn!(
                "Tried to access named cell '{}' which is not defined for Spritesheet '{}'",
                name, self.spritesheet.asset_path
            );
            0
        }
    }

    pub fn position_to_index(&self, row: u32, column: u32) -> u32 {
        if row > self.spritesheet.rows - 1 || column > self.spritesheet.columns - 1 {
            warn!(
                "Incorrect position request ({}, {}) for ({}, {}, {})",
                row,
                column,
                self.spritesheet.asset_path,
                self.spritesheet.rows,
                self.spritesheet.columns
            );
            0
        } else {
            row * self.spritesheet.columns + column
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
