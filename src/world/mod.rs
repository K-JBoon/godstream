use crate::*;
use bevy_ecs_tilemap::prelude::*;

pub mod markers;
pub use markers::*;
pub mod tilesheet;
pub use tilesheet::*;
pub mod stage;
pub use stage::*;

pub fn stage_plugin(app: &mut App) {
    app.add_plugins(TilemapPlugin)
        .add_plugins(RonAssetPlugin::<TilesheetCollection>::new(&[
            "tilesheets.ron",
        ]))
        .add_plugins(RonAssetPlugin::<StageCollection>::new(&["stages.ron"]))
        .configure_loading_state(
            LoadingStateConfig::new(AppState::AssetLoading)
                .load_collection::<TilesheetAssets>()
                .load_collection::<StageAssets>(),
        )
        .add_systems(
            OnEnter(AppState::PrepareStage),
            (despawn::<StageMarker>, spawn_stage),
        );
}
