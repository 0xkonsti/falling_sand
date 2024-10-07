use std::fs::File;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::physic_world::ParticleType;

const COLOR_PALETTE_FILE_PATH: &str = "data/color_palette.ron";

const DEFAULT_BACKGROUND_DARK: &str = "#08090c";
const DEFAULT_TEXT_DARK: &str = "#f5f5f5";
const DEFAULT_TEXT_ALT_DARK: &str = "#8326ed";
const DEFAULT_BACKDROP_LIGHT: &str = "#00000026";

const DEFAULT_BACKGROUND_LIGHT: &str = "#dbdfff";
const DEFAULT_TEXT_LIGHT: &str = "#0a0a0a";
const DEFAULT_TEXT_ALT_LIGHT: &str = "#ba0b4e";
const DEFAULT_BACKDROP_DARK: &str = "#ffffff01";

#[derive(Debug, Serialize, Deserialize)]
struct Colors {
    pub background: String,
    pub text: String,
    pub text_alt: String,
    pub backdrop: String,
}

type ColorGroup = Vec<String>;

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct ColorPalette {
    dark_mode: bool,
    light: Colors,
    dark: Colors,

    sand: ColorGroup,
    water: ColorGroup,
}

impl ColorPalette {
    pub fn load() -> Self {
        if let Ok(file) = File::open(COLOR_PALETTE_FILE_PATH) {
            if let Ok(color_palette) = ron::de::from_reader(file) {
                return color_palette;
            }
        }
        warn!("Failed to open color palette file, using default color palette");
        Self::default()
    }

    pub fn save(&self) {
        ron::ser::to_writer_pretty(
            File::create(COLOR_PALETTE_FILE_PATH).expect("Failed to create color palette file"),
            self,
            Default::default(),
        )
        .expect("Failed to serialize color palette");
    }

    pub fn switch_mode(&mut self) {
        self.dark_mode = !self.dark_mode;
    }

    pub fn background(&self) -> &str {
        &self.colors().background
    }

    pub fn text(&self) -> &str {
        &self.colors().text
    }

    pub fn text_alt(&self) -> &str {
        &self.colors().text_alt
    }

    pub fn backdrop(&self) -> &str {
        &self.colors().backdrop
    }

    pub fn particle_color(&self, particle_type: &ParticleType) -> &ColorGroup {
        match particle_type {
            ParticleType::Sand => &self.sand,
            ParticleType::Water => &self.water,
        }
    }

    fn colors(&self) -> &Colors {
        if self.dark_mode {
            &self.dark
        } else {
            &self.light
        }
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            dark_mode: false,
            light: Colors {
                background: DEFAULT_BACKGROUND_LIGHT.to_string(),
                text: DEFAULT_TEXT_LIGHT.to_string(),
                text_alt: DEFAULT_TEXT_ALT_LIGHT.to_string(),
                backdrop: DEFAULT_BACKDROP_LIGHT.to_string(),
            },
            dark: Colors {
                background: DEFAULT_BACKGROUND_DARK.to_string(),
                text: DEFAULT_TEXT_DARK.to_string(),
                text_alt: DEFAULT_TEXT_ALT_DARK.to_string(),
                backdrop: DEFAULT_BACKDROP_DARK.to_string(),
            },

            sand: vec![
                "#f6d6af".to_string(),
                "#f5d0a9".to_string(),
                "#f4c9a3".to_string(),
                "#f3c29d".to_string(),
                "#f2bb97".to_string(),
            ],
            water: vec![
                "#a3c8ff".to_string(),
                "#8ab8ff".to_string(),
                "#6fa8ff".to_string(),
                "#5b9eff".to_string(),
                "#4b8eff".to_string(),
            ],
        }
    }
}
