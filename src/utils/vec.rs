use std::cmp::Ordering;

use bevy::prelude::*;

pub trait VecParse {
    fn as_vec3(&self) -> Vec3;
}

impl VecParse for IVec2 {
    fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, 0.0)
    }
}

impl VecParse for Vec3 {
    fn as_vec3(&self) -> Vec3 {
        *self
    }
}

pub trait VecOrder {
    fn vec_cmp(&self, other: &Self) -> Ordering;
}

impl VecOrder for IVec2 {
    fn vec_cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then_with(|| self.x.cmp(&other.x))
    }
}
