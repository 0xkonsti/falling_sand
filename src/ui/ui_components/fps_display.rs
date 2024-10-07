use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{events::UpdateColorPaletteEvent, io::ColorPalette, ui::resources::UiFonts, utils::color};

#[derive(Component)]
pub struct FpsDisplay;

pub fn setup(commands: &mut Commands, color_palette: &Res<ColorPalette>, ui_fonts: Res<UiFonts>) {
    commands.spawn((
        TextBundle {
            text: Text::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font_size: 18.,
                        font: ui_fonts.nunito_semi.clone(),
                        color: color::hex_to_color(color_palette.text()).unwrap(),
                    },
                ),
                TextSection::from_style(TextStyle {
                    font_size: 18.,
                    font: ui_fonts.nunito_semi.clone(),
                    color: color::hex_to_color(color_palette.text_alt()).unwrap(),
                }),
            ])
            .with_justify(JustifyText::Center),
            z_index: ZIndex::Global(999), // Always on top
            ..default()
        }
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_content: AlignContent::Center,
            top: Val::Px(8.),
            left: Val::Px(8.),
            padding: UiRect::horizontal(Val::Px(4.)),
            ..default()
        }),
        FpsDisplay,
    ));
}

pub fn update(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsDisplay>>,
    mut update_color_palette: EventReader<UpdateColorPaletteEvent>,
    color_palette: Res<ColorPalette>,
) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }

        for _ in update_color_palette.read() {
            text.sections[0].style.color = color::hex_to_color(color_palette.text()).unwrap();
            text.sections[1].style.color = color::hex_to_color(color_palette.text_alt()).unwrap();
        }
    }
}
