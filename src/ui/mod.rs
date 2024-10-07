mod resources;
mod systems;

mod ui_components;

use bevy::prelude::*;
use ui_components::fps_display;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (systems::load_fonts, systems::setup).chain())
            .add_systems(Update, fps_display::update);
    }
}
