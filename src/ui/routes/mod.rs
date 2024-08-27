pub mod my_route;
pub use my_route::*;

use bevy::prelude::*;

pub struct RoutePlugin;
impl Plugin for RoutePlugin {
    fn build(&self, app: &mut App) {
        app
            // Add each route plugin
            .add_plugins(MyRoutePlugin);
    }
}
