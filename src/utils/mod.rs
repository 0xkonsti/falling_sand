use bevy::prelude::*;

pub mod color;
pub mod vec;

pub fn window_to_grid_position(window_position: Vec2, window_height: f32, grid_size: f32) -> Vec2 {
    let x = window_position.x / grid_size;
    let y = (window_height - window_position.y) / grid_size;

    Vec2::new(x.floor(), y.floor())
}
