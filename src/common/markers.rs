use crate::*;

#[derive(Component)]
pub struct AllyMarker;

#[derive(Component)]
pub struct EnemyMarker;

/// Describes the position of a cell on one half of the battlefield
/// For both sides, position (0, 0) describes the bottom left corner,
/// while position (width - 1, height - 1) describes the top right corner
#[derive(Component, Reflect, Default)]
pub struct CellPosition(pub Vec2);

/// Added to a tile when it's occupied by an entity
#[derive(Component)]
pub struct OccupiedMarker;
