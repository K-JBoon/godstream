use crate::*;

pub struct Tilesheet {
    pub asset_path: String,
    pub tile_width: f32,
    pub tile_height: f32,
    pub columns: u32,
    pub rows: u32,
}

impl Tilesheet {
    pub fn position_to_index(&self, row: u32, column: u32) -> u32 {
        if row > self.rows - 1 || column > self.columns - 1 {
            warn!("Incorrect position request ({}, {}) for ({}, {}, {})", row, column, self.asset_path, self.rows, self.columns);
            0
        } else {
            row * self.columns + column
        }
    }
}
