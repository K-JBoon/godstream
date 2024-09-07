use super::*;

#[derive(Debug, Clone, Reflect, serde::Deserialize)]
pub struct SpritesheetCellPosition(pub u32, pub u32);

#[derive(Component, Debug, Clone, Reflect, serde::Deserialize)]
pub struct Spritesheet {
    pub asset_path: String,
    pub tile_width: f32,
    pub tile_height: f32,
    pub rows: u32,
    pub columns: u32,
    pub named_cells: HashMap<String, SpritesheetCellPosition>,
}
