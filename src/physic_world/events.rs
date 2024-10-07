use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct SpawnParticleEvent {
    pub position: IVec2,
}

impl SpawnParticleEvent {
    pub fn new(position: IVec2) -> Self {
        Self {
            position,
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
