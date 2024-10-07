use bevy::prelude::*;

use super::{resources::UiFonts, ui_components::fps_display};
use crate::io::ColorPalette;

pub fn load_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let nunito_semi: Handle<Font> = asset_server.load("fonts/Nunito-SemiBold.ttf");

    commands.insert_resource(UiFonts {
        nunito_semi,
    });
}

pub fn setup(mut commands: Commands, color_palette: Res<ColorPalette>, ui_fonts: Res<UiFonts>) {
    fps_display::setup(&mut commands, &color_palette, ui_fonts);
}
