use std::{cell::RefCell, rc::Rc};

use glam::Vec3;

use crate::{
    graphics::{Color, Instance, InstanceData, Transform},
    sandbox::{CellKind, Sandbox, sandbox::GRID_SIZE},
};

pub enum BrushSize {
    Small,
    Medium,
    Large,
    Huge,
}

impl BrushSize {
    pub fn next(&self) -> Self {
        match self {
            BrushSize::Small => BrushSize::Medium,
            BrushSize::Medium => BrushSize::Large,
            BrushSize::Large => BrushSize::Huge,
            BrushSize::Huge => BrushSize::Small,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            BrushSize::Small => BrushSize::Huge,
            BrushSize::Medium => BrushSize::Small,
            BrushSize::Large => BrushSize::Medium,
            BrushSize::Huge => BrushSize::Large,
        }
    }

    pub fn offsets(&self) -> Vec<(isize, isize)> {
        match self {
            BrushSize::Small => vec![(0, 0)],
            BrushSize::Medium => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
            BrushSize::Large => vec![(0, 0), (1, 0), (0, 1), (1, 1), (2, 0), (0, 2), (2, 1), (1, 2), (2, 2)],
            BrushSize::Huge => {
                let mut offsets = Vec::new();
                for x in -6..=6 {
                    for y in -6..=6 {
                        offsets.push((x, y));
                    }
                }
                offsets
            }
        }
    }
}

pub struct Brush {
    pub size: BrushSize,
    pub kind: CellKind,
    sandbox:  Rc<RefCell<Sandbox>>,
}

impl Brush {
    pub fn new(sandbox: Rc<RefCell<Sandbox>>) -> Self {
        Self { size: BrushSize::Small, kind: CellKind::Sand, sandbox }
    }

    pub fn spawn(&mut self, pos: (isize, isize)) {
        for offset in self.size.offsets() {
            let grid_pos = (pos.0 + offset.0, pos.1 + offset.1);
            if !self.sandbox.borrow().occupied(&grid_pos) {
                self.sandbox.borrow_mut().insert_cell(grid_pos, self.kind);
            }
        }
    }

    pub fn remove(&mut self, pos: (isize, isize)) {
        for offset in self.size.offsets() {
            let grid_pos = (pos.0 + offset.0, pos.1 + offset.1);
            self.sandbox.borrow_mut().remove_cell(grid_pos);
        }
    }
}
