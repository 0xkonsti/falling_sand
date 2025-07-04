use rand::seq::SliceRandom;

use crate::{graphics::Color, sandbox::sandbox::GridPos};

pub struct MovementOptionGroup(&'static [GridPos]);
pub type CellMovement = &'static [MovementOptionGroup];

impl MovementOptionGroup {
    pub fn shuffled(&self) -> Vec<GridPos> {
        let mut shuffled = self.0.to_vec();
        shuffled.shuffle(&mut rand::rng());
        shuffled
    }
}

#[rustfmt::skip]
const SAND_MOVEMENT: CellMovement = &[
    MovementOptionGroup(&[(0, -1)]),
    MovementOptionGroup(&[(1, -1), (-1, -1)])
];

#[rustfmt::skip]
const WATER_MOVEMENT: CellMovement = &[
    MovementOptionGroup(&[(0, -1)]),
    MovementOptionGroup(&[(1, -1), (-1, -1)]),
    MovementOptionGroup(&[(1, 0), (-1, 0)]),
];

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CellKind {
    Sand,
    Stone,
    Water,
}

impl CellKind {
    pub fn color(&self) -> Color {
        match self {
            CellKind::Sand => Color::hex(0xE3AD6BFF),
            CellKind::Stone => Color::hex(0x3B3B3BFF),
            CellKind::Water => Color::hex(0x4AACE8FF),
        }
    }

    pub fn movement(&self) -> CellMovement {
        match self {
            CellKind::Sand => SAND_MOVEMENT,
            CellKind::Stone => &[],
            CellKind::Water => WATER_MOVEMENT,
        }
    }

    pub fn next_position<'a, L>(&self, pos: GridPos, lookup: L) -> Option<GridPos>
    where
        L: Fn(GridPos) -> Option<&'a Cell>,
    {
        for group in self.movement() {
            let shuffled = group.shuffled();
            for offset in shuffled {
                let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
                if lookup(new_pos).is_none() {
                    return Some(new_pos);
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cell {
    pub kind: CellKind,

    pub idx: usize,
}
