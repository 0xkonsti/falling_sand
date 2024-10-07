use std::fs::File;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};
use serde::{Deserialize, Serialize};

pub const SETTINGS_FILE_PATH: &str = "data/appdata.ron";

#[derive(Debug, Serialize, Deserialize)]
pub enum DataWindowMode {
    Windowed,
    Fullscreen,
    BorderlessFullscreen,
}

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct Settings {
    title: String,
    resolution: (f32, f32),
    window_mode: DataWindowMode,
    vsync: bool,
}

impl Settings {
    pub fn load() -> Self {
        if let Ok(file) = File::open(SETTINGS_FILE_PATH) {
            if let Ok(settings) = ron::de::from_reader(file) {
                return settings;
            }
        }
        warn!("Failed to open settings file, using default settings");
        Self::default()
    }

    pub fn save(&self) {
        ron::ser::to_writer_pretty(
            File::create(SETTINGS_FILE_PATH).expect("Failed to create settings file"),
            self,
            Default::default(),
        )
        .expect("Failed to serialize settings");
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn resolution(&self) -> (f32, f32) {
        self.resolution
    }

    pub fn present_mode(&self) -> PresentMode {
        match self.vsync {
            true => PresentMode::AutoVsync,
            false => PresentMode::Immediate,
        }
    }

    pub fn window_mode(&self) -> WindowMode {
        match self.window_mode {
            DataWindowMode::Windowed => WindowMode::Windowed,
            DataWindowMode::Fullscreen => WindowMode::SizedFullscreen,
            DataWindowMode::BorderlessFullscreen => WindowMode::BorderlessFullscreen,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            title: "Falling Sand".to_string(),
            resolution: (1280., 720.),
            window_mode: DataWindowMode::Windowed,
            vsync: false,
        }
    }
}
