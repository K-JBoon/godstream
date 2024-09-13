use super::*;

type SpritesheetCellLocation = (u32, u32);

#[derive(Hash, Eq, PartialEq, Debug, Clone, Reflect, serde::Deserialize)]
#[serde(untagged)]
pub enum SpritesheetCellIdentifier {
    Position(SpritesheetCellLocation),
    Name(String),
    None,
}

#[derive(Debug, Clone, Reflect, serde::Deserialize)]
#[serde(untagged)]
pub enum SpritesheetCell {
    Animated(Vec<SpritesheetCellIdentifier>),
    Static(SpritesheetCellIdentifier),
}

#[derive(Component, Debug, Clone, Reflect, serde::Deserialize)]
pub struct Spritesheet {
    pub asset_path: String,
    pub tile_width: f32,
    pub tile_height: f32,
    pub rows: u32,
    pub columns: u32,
    pub cell_names: HashMap<String, SpritesheetCell>,
}
