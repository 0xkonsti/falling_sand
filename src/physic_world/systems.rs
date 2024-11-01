use bevy::{prelude::*, window::PrimaryWindow};

use super::{
    events::{DespawnParticleEvent, SpawnParticleEvent},
    particle::ParticleData,
    resources::{CurrentParticleType, CursorParicles},
    world::World,
    ParticleType,
};
use crate::{
    events::UpdateBrushEvent, io::ColorPalette, resources::Brush, systems::PIXELS_PER_UNIT,
    utils::window_to_grid_position,
};

pub fn setup(mut commands: Commands) {
    commands.insert_resource(World::new());
    commands.insert_resource(CurrentParticleType::default());
    commands.insert_resource(CursorParicles::new());
}

// pub fn update() {}

pub fn fixed_update(
    mut world: ResMut<World>,
    mut query: ParamSet<(
        Query<(&mut ParticleType, &mut Transform, &mut ParticleData)>,
        Query<&mut ParticleData>,
    )>,
) {
    world.update(&mut query);
}

pub fn spawn_particle(
    mut commands: Commands,
    mut events: EventReader<SpawnParticleEvent>,
    mut world: ResMut<World>,
    particle_type: Res<CurrentParticleType>,
    color_palette: Res<ColorPalette>,
    brush: Res<Brush>,
) {
    for event in events.read() {
        for offset in brush.current() {
            let position = event.position + *offset;
            if world.occupied(&position) {
                continue;
            }

            let particle_type = match event.particle_type {
                Some(particle_type) => particle_type,
                None => particle_type.0,
            };

            let entity = particle_type.create(&mut commands, &color_palette, position, 1.0);
            world.insert(&position, entity, particle_type);
        }
    }
}

pub fn despawn_particle(
    mut commands: Commands,
    mut events: EventReader<DespawnParticleEvent>,
    mut world: ResMut<World>,
    mut query: Query<&mut ParticleData>,
    brush: Res<Brush>,
) {
    for event in events.read() {
        for offset in brush.current() {
            let position = event.position + *offset;
            if let Some((removed, _)) = world.remove(&position, &mut query) {
                commands.entity(removed).despawn();
            }
        }
    }
}

pub fn key_input(
    mut current_particle_type: ResMut<CurrentParticleType>,
    mut brush_event: EventWriter<UpdateBrushEvent>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut update = true;

    if keys.just_pressed(KeyCode::Digit1) && current_particle_type.0 != ParticleType::Sand {
        current_particle_type.0 = ParticleType::Sand;
    } else if keys.just_pressed(KeyCode::Digit2) && current_particle_type.0 != ParticleType::Water {
        current_particle_type.0 = ParticleType::Water;
    } else {
        update = false;
    }

    if update {
        brush_event.send(UpdateBrushEvent);
    }
}

pub fn mouse_input(
    mut spawn_events: EventWriter<SpawnParticleEvent>,
    mut despawn_events: EventWriter<DespawnParticleEvent>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if !mouse_button_input.any_pressed([MouseButton::Left, MouseButton::Right]) {
        return;
    }

    if let Ok(window) = window_query.get_single() {
        if let Some(position) = window.cursor_position() {
            let position = window_to_grid_position(position, window.height(), PIXELS_PER_UNIT).as_ivec2();

            if mouse_button_input.pressed(MouseButton::Left) {
                spawn_events.send(SpawnParticleEvent::new(position, None));
            } else if mouse_button_input.pressed(MouseButton::Right) {
                despawn_events.send(DespawnParticleEvent::new(position));
            }
        }
    }
}

pub fn cursor_particle(
    mut commands: Commands,
    mut cursor_particles: ResMut<CursorParicles>,
    current_particle_type: Res<CurrentParticleType>,
    brush: Res<Brush>,
    color_palette: Res<ColorPalette>,
    mut particle_query: Query<&mut Transform, With<ParticleType>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut cursor_moved: EventReader<CursorMoved>,
    mut brush_event: EventReader<UpdateBrushEvent>,
) {
    let window = window_query.single();

    for e in cursor_moved.read() {
        let position = window_to_grid_position(e.position, window.height(), PIXELS_PER_UNIT);

        if cursor_particles.0.is_empty() {
            for offset in brush.current() {
                let offset_position = position + (*offset).as_vec2();
                let entity = current_particle_type.create_transparent_particle(
                    &mut commands,
                    &color_palette,
                    offset_position.as_ivec2(),
                );
                cursor_particles.0.push(entity);
            }

            continue;
        }

        for (i, entity) in cursor_particles.0.iter().enumerate() {
            if let Ok(mut transform) = particle_query.get_mut(*entity) {
                if i >= brush.current().len() {
                    continue;
                }
                let offset_position = position + brush.current()[i].as_vec2();
                transform.translation = Vec3::new(offset_position.x, offset_position.y, 0.);
            }
        }
    }

    for _ in brush_event.read() {
        cursor_particles.clear(&mut commands);

        if let Some(cursor_position) = window.cursor_position() {
            let position =
                window_to_grid_position(cursor_position, window.height(), PIXELS_PER_UNIT).as_ivec2();

            for offset in brush.current() {
                let offset_position = position + (*offset);
                let entity = current_particle_type.create_transparent_particle(
                    &mut commands,
                    &color_palette,
                    offset_position,
                );
                cursor_particles.0.push(entity);
            }
        }
    }
}
