use crate::*;

pub mod markers;
pub use markers::*;
pub mod resources;
pub use resources::*;
pub mod pantheon;
pub use pantheon::*;
pub mod domain;
pub use domain::*;
pub mod ally_unit;
pub use ally_unit::*;

pub fn unit_plugin(app: &mut App) {
    app.register_type::<AllyUnit>()
        .add_plugins(RonAssetPlugin::<AllyUnit>::new(&["ally_unit.ron"]))
        .add_systems(Startup, load_ally_units)
        .add_systems(
            Update,
            (spawn_ally_unit.run_if(input_just_pressed(KeyCode::KeyQ)),),
        );
}
