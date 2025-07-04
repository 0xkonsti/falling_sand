use glam::Vec3;
use hashbrown::HashMap;

use crate::{
    graphics::{Instance, InstanceData, Transform},
    sandbox::cell::{Cell, CellKind, TransitionTarget},
};

pub const GRID_SIZE: f32 = 32.0;
const UPDATE_RATE: f64 = 1.0 / 24.0; // 60 FPS
const GRAVITY: f32 = 5.0; // Gravity effect on cell movement
const MOMENTUM_THRESHOLD: f32 = 75.0; // Threshold for momentum to be considered irrelevant

pub type GridPos = (isize, isize);

#[derive(Debug, Clone, PartialEq)]
pub struct Sandbox {
    grid:         HashMap<GridPos, Cell>,
    active_cells: Vec<GridPos>,

    mesh_instance: Instance,

    time_since_last_update: f64,
}

impl Sandbox {
    pub fn new(mesh_instance: Instance) -> Self {
        Self { grid: HashMap::new(), active_cells: Vec::new(), mesh_instance, time_since_last_update: 0.0 }
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
        let cell = Cell::new(cell_kind, self.mesh_instance.instance_count());
        self.grid.insert(pos, cell);
        self.active_cells.push(pos);
        self.add_instance(&pos, &cell);
    }

    pub fn remove_cell(&mut self, pos: GridPos) -> Option<Cell> {
        if let Some(cell) = self.grid.remove(&pos) {
            self.active_cells.retain(|&p| p != pos);
            self.mesh_instance.remove_instance(cell.idx);
            for neighbour in self.get_neighbourins(&pos) {
                if let Some(neighbour_cell) = self.grid.get_mut(&neighbour) {
                    if !neighbour_cell.sleeping {
                        continue;
                    }
                    neighbour_cell.wake();
                    self.active_cells.push(neighbour);
                }
            }
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
            self.active_cells.retain(|&p| p != *from);
            self.active_cells.push(*to);
            let instance = self.mesh_instance.get_instance(cell.idx).expect("Instance not found");
            let mut transform = instance.as_transform();
            transform.translation = Vec3::new(to.0 as f32 * GRID_SIZE, to.1 as f32 * GRID_SIZE, 0.0);

            self.mesh_instance.update_instance_transform(cell.idx, transform);
        }
    }

    pub fn swap_cells(&mut self, pos1: &GridPos, pos2: GridPos) {
        let cell1_kind = self.grid.get(pos1).map(|c| c.kind);
        let cell2_kind = self.grid.get(&pos2).map(|c| c.kind);

        if cell1_kind.is_none() || cell2_kind.is_none() {
            return; // One of the cells does not exist
        }

        self.change_cell_kind(*pos1, cell2_kind.unwrap());
        self.change_cell_kind(pos2, cell1_kind.unwrap());
    }

    pub fn change_cell_kind(&mut self, pos: GridPos, new_kind: CellKind) {
        if let Some(cell) = self.grid.get_mut(&pos) {
            cell.kind = new_kind;
            self.mesh_instance.update_instance_color(cell.idx, &new_kind.color());
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.time_since_last_update += dt;
        if self.time_since_last_update < UPDATE_RATE {
            return;
        }
        self.time_since_last_update -= UPDATE_RATE;

        // let keys: Vec<GridPos> = self.grid.keys().cloned().collect();
        let keys = self.active_cells.clone();

        for pos in keys {
            let cell = self.grid.get(&pos).expect("Cell not found");
            if cell.momentum.abs() > MOMENTUM_THRESHOLD {
                self.remove_cell(pos);
                continue; // Skip cells with too much momentum
            }
            let update = cell.update(pos, |p| self.get_cell(p), GRAVITY * UPDATE_RATE as f32);
            if !update.updated {
                if let Some(cell) = self.grid.get_mut(&pos) {
                    cell.sleep();
                    if cell.sleeping {
                        self.active_cells.retain(|&p| p != pos);
                    }
                }
                continue;
            }

            if update.swapped {
                self.swap_cells(&pos, update.new_pos.unwrap());
            } else if let Some(transition) = update.transition {
                if transition.target == TransitionTarget::This {
                    self.change_cell_kind(pos, transition.result);
                } else if let Some(new_pos) = update.new_pos {
                    self.change_cell_kind(new_pos, transition.result);
                }
                if transition.remove {
                    self.remove_cell(pos);
                }
            } else if let Some(new_pos) = update.new_pos {
                self.move_cell(&pos, &new_pos);
            }

            if let Some(new_pos) = update.new_pos
                && let Some(cell) = self.grid.get_mut(&new_pos)
            {
                cell.momentum = update.new_momentum;
                cell.wake();
                // wake up neighbours
                for neighbour in self.get_neighbourins(&new_pos) {
                    if let Some(neighbour_cell) = self.grid.get_mut(&neighbour) {
                        if !neighbour_cell.sleeping {
                            continue;
                        }
                        neighbour_cell.wake();
                        self.active_cells.push(neighbour);
                    }
                }
            }

            if let Some(cell) = self.grid.get_mut(&pos) {
                cell.wake();
                // wake up neighbours
                for neighbour in self.get_neighbourins(&pos) {
                    if let Some(neighbour_cell) = self.grid.get_mut(&neighbour) {
                        if !neighbour_cell.sleeping {
                            continue;
                        }
                        neighbour_cell.wake();
                        self.active_cells.push(neighbour);
                    }
                }
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

    fn get_neighbourins(&self, pos: &GridPos) -> Vec<GridPos> {
        let mut neighbours = Vec::new();
        for dx in -3..=3 {
            for dy in -3..=3 {
                if dx == 0 && dy == 0 {
                    continue; // Skip the current cell
                }
                let neighbour_pos = (pos.0 + dx, pos.1 + dy);
                if self.grid.contains_key(&neighbour_pos) {
                    neighbours.push(neighbour_pos);
                }
            }
        }
        neighbours
    }
}
