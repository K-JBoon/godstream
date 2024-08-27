use crate::*;

pub mod tile;
pub use tile::*;
pub mod stage;
pub use stage::*;

// Marker structs
#[derive(Component)]
pub struct VoxelBackground;

#[derive(Component)]
pub struct VoxelStage;
