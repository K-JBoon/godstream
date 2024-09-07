use crate::*;

pub mod constants;
pub use constants::*;
pub mod markers;
pub use markers::*;
pub mod resources;
pub use resources::*;
pub mod spritesheet;
pub use spritesheet::*;

pub fn despawn<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_common_resources(mut commands: Commands) {
    commands.insert_resource(HoveredCellPosition(CellPosition(Vec2::new(0.0, 0.0))));
}

pub fn common_plugin(app: &mut App) {
    app.add_systems(Startup, setup_common_resources);

    app.register_type::<CellPosition>();
}
