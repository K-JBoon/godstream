use crate::*;

pub enum Tile {
    GrassBlock,
    WaterGrassBlock,
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Tile::GrassBlock => "voxels/tiles.vox#GrassBlock".into(),
            Tile::WaterGrassBlock => "voxels/tiles.vox#WaterGrassBlock".into(),
        }
    }
}
