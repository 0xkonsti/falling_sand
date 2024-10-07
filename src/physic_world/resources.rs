use bevy::prelude::*;

use super::ParticleType;
use crate::io::ColorPalette;

#[derive(Resource, Deref, DerefMut)]
pub struct CurrentParticleType(pub ParticleType);

impl CurrentParticleType {
    pub fn create_particle(
        &self,
        commands: &mut Commands,
        color_palette: &Res<ColorPalette>,
        position: IVec2,
    ) -> Entity {
        self.0.create(commands, color_palette, position, 1.)
    }

    pub fn create_transparent_particle(
        &self,
        commands: &mut Commands,
        color_palette: &Res<ColorPalette>,
        position: IVec2,
    ) -> Entity {
        self.0.create(commands, color_palette, position, 0.5)
    }
}

impl Default for CurrentParticleType {
    fn default() -> Self {
        Self(ParticleType::Sand)
    }
}

#[derive(Resource)]
pub struct CursorParicles(pub Vec<Entity>);

impl CursorParicles {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn clear(&mut self, commands: &mut Commands) {
        for entity in self.0.iter() {
            commands.entity(*entity).despawn();
        }
        self.0.clear();
    }
}
