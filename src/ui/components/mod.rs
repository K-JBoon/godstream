pub mod main_button;
pub use main_button::*;

use bevy::prelude::*;

pub struct ComponentPlugin;
impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add each component plugin
            .add_plugins(MainButtonPlugin);
    }
}
