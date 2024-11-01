use std::cmp::Ordering;

use bevy::{input::mouse::MouseWheel, prelude::*, window::PrimaryWindow};

use crate::{
    components::MainCamera,
    events::{UpdateBrushEvent, UpdateColorPaletteEvent},
    io::{ColorPalette, Settings},
    resources::Brush,
    utils::color,
};

pub const PIXELS_PER_UNIT: f32 = 16.0;

pub fn setup(mut commands: Commands, mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.single_mut();

    let settings = Settings::load();
    let color_palette = ColorPalette::load();

    window_setup(&mut window, &settings);

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(
                window.width() / (2.0 * PIXELS_PER_UNIT),
                window.height() / (2.0 * PIXELS_PER_UNIT),
                999.,
            ),
            camera: Camera {
                clear_color: ClearColorConfig::Custom(
                    color::hex_to_color(color_palette.background()).unwrap(),
                ),
                ..default()
            },
            projection: OrthographicProjection {
                scaling_mode: bevy::render::camera::ScalingMode::WindowSize(PIXELS_PER_UNIT),
                ..default()
            },
            ..default()
        },
        MainCamera,
    ));

    commands.insert_resource(settings);
    commands.insert_resource(color_palette);
    commands.insert_resource(Brush::default());
}

pub fn cleanup(mut exit: EventReader<AppExit>, settings: Res<Settings>, color_palette: Res<ColorPalette>) {
    for e in exit.read() {
        if e == &AppExit::Success {
            info!("Running cleanup...");

            settings.save();
            color_palette.save();

            info!("Cleanup done");
        }
    }
}

pub fn debug_exit(mut exit: EventWriter<AppExit>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        info!("ESC pressed, exiting...");
        exit.send(AppExit::Success);
    }
}

pub fn switch_color_palette(
    keys: Res<ButtonInput<KeyCode>>,
    mut color_palette: ResMut<ColorPalette>,
    mut camera_query: Query<&mut Camera, With<MainCamera>>,
    mut update_color_palette: EventWriter<UpdateColorPaletteEvent>,
) {
    if keys.just_pressed(KeyCode::F1) {
        color_palette.switch_mode();

        let mut camera = camera_query.single_mut();

        camera.clear_color =
            ClearColorConfig::Custom(color::hex_to_color(color_palette.background()).unwrap());

        update_color_palette.send(UpdateColorPaletteEvent);
    }
}

pub fn change_brush(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut brush_event: EventWriter<UpdateBrushEvent>,
    mut brush: ResMut<Brush>,
) {
    for event in mouse_wheel.read() {
        let delta = event.y as i32;

        match delta.cmp(&0) {
            Ordering::Greater => brush.next(),
            Ordering::Less => brush.previous(),
            _ => (),
        }

        brush_event.send(UpdateBrushEvent);
    }
}

fn window_setup(window: &mut Window, settings: &Settings) {
    window.title = settings.title();

    window.resolution = settings.resolution().into();

    window.present_mode = settings.present_mode();
    window.mode = settings.window_mode();
}
