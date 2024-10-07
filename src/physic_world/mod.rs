use bevy::prelude::*;

mod events;
mod particle;
mod resources;
mod systems;
mod world;

pub use particle::ParticleType;

pub struct PhysicsWorldPlugin;

impl Plugin for PhysicsWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::SpawnParticleEvent>()
            .add_event::<events::DespawnParticleEvent>()
            .add_systems(Startup, systems::setup)
            .add_systems(
                PreUpdate,
                (
                    (systems::cursor_particle, systems::mouse_input, systems::key_input).chain(),
                    (systems::spawn_particle, systems::despawn_particle).chain(),
                ),
            )
            .add_systems(FixedUpdate, systems::fixed_update);
    }
}
