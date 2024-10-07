use bevy::{prelude::*, utils::HashMap};

use super::{particle::ParticleData, ParticleType};
use crate::utils::vec::{VecOrder, VecParse};

const GRAVITY: f32 = -0.25;
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
    grid: HashMap<IVec2, Entity>,
    sorted: Vec<IVec2>,
}

impl World {
    pub fn new() -> Self {
        Self {
            grid: HashMap::default(),
            sorted: Vec::new(),
        }
    }

    pub fn get(&self, position: &IVec2) -> Option<&Entity> {
        self.grid.get(position)
    }

    pub fn insert(&mut self, position: &IVec2, entity: Entity) {
        self.grid.insert(*position, entity);
        match self.sorted.binary_search_by(|&x| x.vec_cmp(position)) {
            Ok(_) => {}
            Err(index) => self.sorted.insert(index, *position),
        }
    }

    pub fn remove(&mut self, position: &IVec2) -> Option<Entity> {
        if let Some(removed) = self.grid.remove(position) {
            self.sorted.retain(|x| x != position);
            Some(removed)
        } else {
            None
        }
    }

    pub fn move_entity(&mut self, from: &IVec2, to: &IVec2) {
        if let Some(entity) = self.remove(from) {
            self.insert(to, entity);
        }
    }

    pub fn occupied(&self, position: &IVec2) -> bool {
        self.get(position).is_some()
    }

    pub fn update(&mut self, particles: &mut Query<(&ParticleType, &mut Transform, &mut ParticleData)>) {
        let mut new_grid = self.grid.clone();
        let mut to_move = Vec::new();

        for position in self.sorted.iter().copied() {
            if let Some(entity) = self.get(&position) {
                if let Ok((particle_type, mut transform, mut data)) = particles.get_mut(*entity) {
                    if data.asleep() {
                        continue;
                    }

                    data.accelerate(Vec2::new(0.0, GRAVITY));

                    if let Some(new_position) =
                        particle_type.next_position(&position, &mut data, |p| new_grid.get(p).copied())
                    {
                        new_grid.remove(&position);
                        new_grid.insert(new_position, *entity);
                        transform.translation = new_position.as_vec3();

                        to_move.push((position, new_position));

                        // Refresh/wake all neighbors
                        for neighbor in self.get_neighbors(&position) {
                            if let Ok((_, _, mut neighbor_data)) = particles.get_mut(neighbor) {
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
            self.move_entity(&from, &to);
        }
    }

    fn get_neighbors(&self, position: &IVec2) -> Vec<Entity> {
        NEIGHBORS
            .iter()
            .filter_map(|offset| self.get(&(*position + *offset)).copied())
            .collect()
    }
}
