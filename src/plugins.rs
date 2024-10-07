use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};

const LOG_LEVEL: Level = Level::INFO;

pub struct Defaults;

impl Plugin for Defaults {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: LOG_LEVEL,
                    ..default()
                }),
        );
    }
}
