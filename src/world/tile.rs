use std::fmt;

pub enum Tile {
    BlockGrass,
    BattlefieldBlockAllyStone,
    BattlefieldBlockEnemyStone,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str: &str = match self {
            Tile::BlockGrass => "voxels/tiles.vox#BlockGrass",
            Tile::BattlefieldBlockAllyStone => "voxels/tiles.vox#BattlefieldBlockAllyStone",
            Tile::BattlefieldBlockEnemyStone => "voxels/tiles.vox#BattlefieldBlockEnemyStone",
        };

        write!(f, "{}", str)
    }
}
