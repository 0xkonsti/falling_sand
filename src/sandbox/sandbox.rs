use glam::Vec3;
use hashbrown::HashMap;

use crate::{
    graphics::{Color, Instance, InstanceData, Transform},
    sandbox::cell::{Cell, CellKind},
};

pub const GRID_SIZE: f32 = 32.0;
const UPDATE_RATE: f64 = 1.0 / 24.0; // 60 FPS

pub type GridPos = (isize, isize);

#[derive(Debug, Clone, PartialEq)]
pub struct Sandbox {
    grid: HashMap<GridPos, Cell>,

    mesh_instance: Instance,

    time_since_last_update: f64,
}

impl Sandbox {
    pub fn new(mesh_instance: Instance) -> Self {
        Self { grid: HashMap::new(), mesh_instance, time_since_last_update: 0.0 }
    }

    pub fn grid_pos_from_world_pos(world_pos: Vec3) -> GridPos {
        let x = Self::to_grid_coord(world_pos.x);
        let y = Self::to_grid_coord(world_pos.y);
        (x, y)
    }

    pub fn draw(&mut self) {
        self.mesh_instance.draw();
    }

    pub fn get_cell(&self, pos: GridPos) -> Option<&Cell> {
        self.grid.get(&pos)
    }

    pub fn occupied(&self, pos: &GridPos) -> bool {
        self.grid.contains_key(pos)
    }

    pub fn insert_cell(&mut self, pos: GridPos, cell_kind: CellKind) {
        if self.occupied(&pos) {
            return;
        }
        let cell = Cell { kind: cell_kind, idx: self.mesh_instance.instance_count() };
        self.grid.insert(pos, cell);
        self.add_instance(&pos, &cell);
    }

    pub fn remove_cell(&mut self, pos: GridPos) -> Option<Cell> {
        if let Some(cell) = self.grid.remove(&pos) {
            self.mesh_instance.remove_instance(cell.idx);
            let keys: Vec<GridPos> = self.grid.keys().cloned().collect();
            for key in keys {
                if let Some(c) = self.grid.get_mut(&key) {
                    if c.idx > cell.idx {
                        c.idx -= 1; // Adjust indices of remaining cells
                    }
                }
            }
            return Some(cell);
        }
        None
    }

    pub fn move_cell(&mut self, from: &GridPos, to: &GridPos) {
        if let Some(cell) = self.grid.remove(from) {
            self.grid.insert(*to, cell);
            let instance = self.mesh_instance.get_instance(cell.idx).expect("Instance not found");
            let mut transform = instance.as_transform();
            transform.translation = Vec3::new(to.0 as f32 * GRID_SIZE, to.1 as f32 * GRID_SIZE, 0.0);

            self.mesh_instance.update_instance_transform(cell.idx, transform);
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.time_since_last_update += dt;
        if self.time_since_last_update < UPDATE_RATE {
            return;
        }
        self.time_since_last_update -= UPDATE_RATE;

        let keys: Vec<GridPos> = self.grid.keys().cloned().collect();

        for pos in keys {
            if let Some(cell) = self.grid.get(&pos) {
                let Some(new_pos) = cell.kind.next_position(pos, |p| self.get_cell(p)) else {
                    continue;
                };
                self.move_cell(&pos, &new_pos);
            }
        }
    }

    // ----------------< Private >----------------
    fn add_instance(&mut self, pos: &GridPos, cell: &Cell) {
        let transform = Transform::from_translation(Vec3::new(
            pos.0 as f32 * GRID_SIZE as f32,
            pos.1 as f32 * GRID_SIZE as f32,
            0.0,
        ))
        .with_scale(Vec3::splat(GRID_SIZE as f32));

        self.mesh_instance.add_instance(InstanceData::new(transform, cell.kind.color()));
    }

    fn to_grid_coord(value: f32) -> isize {
        if value < 0.0 { (value / GRID_SIZE - 0.5) as isize } else { (value / GRID_SIZE + 0.5) as isize }
    }
}
