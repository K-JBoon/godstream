use crate::*;

pub mod markers;
pub use markers::*;
pub mod resources;
pub use resources::*;
pub mod tilesheet;
pub use tilesheet::*;
pub mod stage;
pub use stage::*;

pub fn stage_plugin(app: &mut App) {
    app.insert_resource(CurrentStage {
        stage: Stage::ArenaBase,
    })
    .add_systems(
        OnEnter(AppState::LoadingScreen),
        (despawn::<StageMarker>, spawn_stage),
    );
}
