use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

mod io;
mod physic_world;
mod ui;
mod utils;

mod components;
mod events;
mod plugins;
mod resources;
mod systems;

use events::{UpdateBrushEvent, UpdateColorPaletteEvent};
use physic_world::PhysicsWorldPlugin;
use plugins::Defaults;
use systems::{change_brush, cleanup, debug_exit, setup, switch_color_palette};
use ui::UiPlugin;

fn main() {
    App::new()
        .add_event::<UpdateColorPaletteEvent>()
        .add_event::<UpdateBrushEvent>()
        .add_plugins((Defaults, PhysicsWorldPlugin, FrameTimeDiagnosticsPlugin, UiPlugin))
        .add_systems(PreStartup, setup)
        .add_systems(PreUpdate, (debug_exit, switch_color_palette, change_brush))
        .add_systems(PostUpdate, cleanup)
        .run();
}
