use bevy::{prelude::*, utils::HashMap};

use super::{
    events::{DespawnParticleEvent, SpawnParticleEvent},
    particle::ParticleData,
    ParticleType,
};
use crate::utils::vec::{VecOrder, VecParse};

const GRAVITY: f32 = -0.225;
const NEIGHBORS: [IVec2; 8] = [
    IVec2::new(0, 1),
    IVec2::new(0, -1),
    IVec2::new(1, 0),
    IVec2::new(-1, 0),
    IVec2::new(1, 1),
    IVec2::new(1, -1),
    IVec2::new(-1, 1),
    IVec2::new(-1, -1),
];

#[derive(Debug, Resource)]
pub struct World {
    grid: HashMap<IVec2, (Entity, ParticleType)>,
    sorted: Vec<IVec2>,
}

impl World {
    pub fn new() -> Self {
        Self {
            grid: HashMap::default(),
            sorted: Vec::new(),
        }
    }

    pub fn get(&self, position: &IVec2) -> Option<&(Entity, ParticleType)> {
        self.grid.get(position)
    }

    pub fn insert(&mut self, position: &IVec2, entity: Entity, particle_type: ParticleType) {
        self.grid.insert(*position, (entity, particle_type));
        match self.sorted.binary_search_by(|&x| x.vec_cmp(position)) {
            Ok(_) => {}
            Err(index) => self.sorted.insert(index, *position),
        }
    }

    pub fn remove(
        &mut self,
        position: &IVec2,
        particles: &mut Query<&mut ParticleData>,
    ) -> Option<(Entity, ParticleType)> {
        if let Some(removed) = self.grid.remove(position) {
            self.sorted.retain(|x| x != position);

            for (neighbor, _) in self.get_neighbors(position) {
                if let Ok(mut data) = particles.get_mut(neighbor) {
                    data.wake();
                }
            }

            Some(removed)
        } else {
            None
        }
    }

    pub fn move_entity(&mut self, from: &IVec2, to: &IVec2, particles: &mut Query<&mut ParticleData>) {
        if let Some((entity, particle_type)) = self.remove(from, particles) {
            self.insert(to, entity, particle_type);
        }
    }

    pub fn occupied(&self, position: &IVec2) -> bool {
        self.get(position).is_some()
    }

    pub fn update(
        &mut self,
        particles: &mut ParamSet<(
            Query<(&mut ParticleType, &mut Transform, &mut ParticleData)>,
            Query<&mut ParticleData>,
        )>,
    ) {
        let mut new_grid = self.grid.clone();
        let mut to_move = Vec::new();

        for position in self.sorted.iter().copied() {
            if let Some((entity, entity_type)) = self.get(&position) {
                if let Ok((particle_type, mut transform, mut data)) = particles.p0().get_mut(*entity) {
                    if data.asleep() {
                        continue;
                    }

                    data.accelerate(GRAVITY);

                    if let Some(new_position) =
                        particle_type.next_position(&position, &mut data, |p| new_grid.get(p).copied())
                    {
                        new_grid.remove(&position);
                        new_grid.insert(new_position, (*entity, *particle_type));
                        transform.translation = new_position.as_vec3();

                        to_move.push((position, new_position));

                        // Refresh/wake all neighbors
                        for (neighbor, neighbor_type) in self.get_neighbors(&position) {
                            if let Ok((_, neighbor_pos, mut neighbor_data)) = particles.p0().get_mut(neighbor)
                            {
                                let neighbor_pos = neighbor_pos.translation.truncate().as_ivec2();

                                match entity_type {
                                    ParticleType::Water => match neighbor_type {
                                        ParticleType::Sand => {
                                            if neighbor_data.stationary() {
                                                //new_grid.remove(&neighbor_pos);
                                                //new_grid.remove(&new_position);
                                                //
                                                //new_grid
                                                //    .insert(neighbor_pos, (neighbor, ParticleType::WetSand));
                                            }
                                        }
                                        _ => {}
                                    },
                                    _ => {}
                                }

                                neighbor_data.wake();
                            }
                        }
                    } else {
                        data.sleep();
                    }
                }
            }
        }

        for (from, to) in to_move {
            self.move_entity(&from, &to, &mut particles.p1());
        }
    }

    fn get_neighbors(&self, position: &IVec2) -> Vec<(Entity, ParticleType)> {
        NEIGHBORS
            .iter()
            .filter_map(|offset| self.get(&(*position + *offset)).copied())
            .collect()
    }
}
