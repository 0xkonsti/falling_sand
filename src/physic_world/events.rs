use bevy::prelude::*;

use super::ParticleType;

#[derive(Event, Debug)]
pub struct SpawnParticleEvent {
    pub position: IVec2,
    pub particle_type: Option<ParticleType>,
}

impl SpawnParticleEvent {
    pub fn new(position: IVec2, particle_type: Option<ParticleType>) -> Self {
        Self {
            position,
            particle_type,
        }
    }
}

#[derive(Event, Debug)]
pub struct DespawnParticleEvent {
    pub position: IVec2,
}

impl DespawnParticleEvent {
    pub fn new(position: IVec2) -> Self {
        Self {
            position,
        }
    }
}
