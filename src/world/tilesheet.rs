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
        if let Some(cell) = self.spritesheet.named_cells.get(name) {
            self.position_to_index(cell.0, cell.1)
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
