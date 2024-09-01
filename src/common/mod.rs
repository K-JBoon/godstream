use crate::*;

pub mod constants;
pub use constants::*;
pub mod markers;
pub use markers::*;
pub mod resources;
pub use resources::*;

pub fn despawn<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
