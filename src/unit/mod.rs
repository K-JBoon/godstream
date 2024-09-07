use crate::*;

pub mod markers;
pub use markers::*;
pub mod pantheon;
pub use pantheon::*;
pub mod domain;
pub use domain::*;
pub mod unit;
pub use unit::*;

pub fn unit_plugin(app: &mut App) {
    app.register_type::<Unit>()
        .add_plugins(RonAssetPlugin::<UnitCollection>::new(&["units.ron"]))
        .add_systems(Startup, load_units)
        .add_systems(
            Update,
            (spawn_unit.run_if(input_just_pressed(KeyCode::KeyQ)),),
        );
}
