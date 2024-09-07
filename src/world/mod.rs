use crate::*;
use bevy_ecs_tilemap::prelude::*;

pub mod markers;
pub use markers::*;
pub mod resources;
pub use resources::*;
pub mod tilesheet;
pub use tilesheet::*;
pub mod stage;
pub use stage::*;

pub fn stage_plugin(app: &mut App) {
    app.add_plugins(TilemapPlugin)
        .insert_resource(CurrentStage {
            stage: Stage::ArenaBase,
        })
        .add_systems(
            OnEnter(AppState::LoadingScreen),
            (despawn::<StageMarker>, spawn_stage),
        );
}
