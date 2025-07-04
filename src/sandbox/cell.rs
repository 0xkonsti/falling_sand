use rand::seq::{IndexedRandom, SliceRandom};

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
const WET_SAND_MOVEMENT: CellMovement = &[
    MovementOptionGroup(&[(0, -1)]),
    MovementOptionGroup(&[(1, -1), (-1, -1)])
];

#[rustfmt::skip]
const WATER_MOVEMENT: CellMovement = &[
    MovementOptionGroup(&[(0, -1)]),
    MovementOptionGroup(&[(1, -1), (-1, -1)]),
    MovementOptionGroup(&[(1, 0), (-1, 0)]),
];

const SAND_COLOR: [Color; 5] = [
    Color::new(0.965, 0.843, 0.690, 1.0),
    Color::new(0.949, 0.824, 0.663, 1.0),
    Color::new(0.925, 0.800, 0.635, 1.0),
    Color::new(0.906, 0.769, 0.588, 1.0),
    Color::new(0.882, 0.749, 0.573, 1.0),
];

const WET_SAND_COLOR: [Color; 5] = [
    Color::new(0.929, 0.694, 0.392, 1.0),
    Color::new(0.906, 0.678, 0.384, 1.0),
    Color::new(0.871, 0.659, 0.376, 1.0),
    Color::new(0.851, 0.631, 0.345, 1.0),
    Color::new(0.820, 0.616, 0.345, 1.0),
];

const STONE_COLOR: [Color; 5] = [
    Color::new(0.313, 0.313, 0.313, 1.0),
    Color::new(0.345, 0.345, 0.345, 1.0),
    Color::new(0.392, 0.392, 0.392, 1.0),
    Color::new(0.254, 0.254, 0.254, 1.0),
    Color::new(0.196, 0.196, 0.196, 1.0),
];

const WATER_COLOR: [Color; 5] = [
    Color::new(0.000, 0.624, 0.784, 1.0),
    Color::new(0.000, 0.671, 0.843, 1.0),
    Color::new(0.000, 0.710, 0.894, 1.0),
    Color::new(0.122, 0.757, 0.918, 1.0),
    Color::new(0.224, 0.816, 0.969, 1.0),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TransitionTarget {
    This,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CellTransition {
    condition:  CellKind,
    pub result: CellKind,
    pub remove: bool,
    pub target: TransitionTarget,
}

const SAND_TRANSITIONS: &[CellTransition] = &[CellTransition {
    condition: CellKind::Water,
    result:    CellKind::WetSand,
    remove:    true,
    target:    TransitionTarget::Other,
}];

const WET_SAND_TRANSITIONS: &[CellTransition] = &[];

const STONE_TRANSITIONS: &[CellTransition] = &[];

const WATER_TRANSITIONS: &[CellTransition] = &[CellTransition {
    condition: CellKind::Sand,
    result:    CellKind::WetSand,
    remove:    true,
    target:    TransitionTarget::Other,
}];

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CellKind {
    Sand,
    WetSand,
    Stone,
    Water,
}

impl CellKind {
    pub fn color(&self) -> &Color {
        match self {
            CellKind::Sand => &SAND_COLOR,
            CellKind::WetSand => &WET_SAND_COLOR,
            CellKind::Stone => &STONE_COLOR,
            CellKind::Water => &WATER_COLOR,
        }
        .choose(&mut rand::rng())
        .unwrap()
    }

    pub fn movement(&self) -> CellMovement {
        match self {
            CellKind::Sand => SAND_MOVEMENT,
            CellKind::WetSand => WET_SAND_MOVEMENT,
            CellKind::Stone => &[],
            CellKind::Water => WATER_MOVEMENT,
        }
    }

    pub fn transitions(&self) -> &[CellTransition] {
        match self {
            CellKind::Sand => SAND_TRANSITIONS,
            CellKind::WetSand => WET_SAND_TRANSITIONS,
            CellKind::Stone => STONE_TRANSITIONS,
            CellKind::Water => WATER_TRANSITIONS,
        }
    }

    pub fn is_liquid(&self) -> bool {
        matches!(self, CellKind::Water)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CellUpdate {
    pub updated:      bool,
    pub new_pos:      Option<GridPos>,
    pub new_momentum: f32,
    pub transition:   Option<CellTransition>,
    pub swapped:      bool,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Cell {
    pub kind: CellKind,
    pub idx:  usize,

    pub momentum: f32,
}

impl Cell {
    pub fn new(kind: CellKind, idx: usize) -> Self {
        Self { kind, idx, momentum: 0.0 }
    }

    pub fn update<'a, L>(&self, pos: GridPos, lookup: L, acceleration: f32) -> CellUpdate
    where
        L: Fn(GridPos) -> Option<&'a Cell>,
    {
        let mut update = CellUpdate {
            updated:      false,
            new_pos:      None,
            new_momentum: self.momentum + acceleration,
            transition:   None,
            swapped:      false,
        };

        let transitions = self.kind.transitions();

        let mut tmp_pos = pos;
        let mut momentum = update.new_momentum;
        let mut last_dir = (0, 0);

        while momentum > 0.0 {
            let mut dead_end = true;
            for group in self.kind.movement() {
                let mut shuffled = group.shuffled();
                shuffled.insert(0, last_dir); // Try to follow the last direction first
                for offset in shuffled {
                    let new_pos = (tmp_pos.0 + offset.0, tmp_pos.1 + offset.1);
                    let Some(collider) = lookup(new_pos) else {
                        update.updated = true;
                        update.new_pos = Some(new_pos);
                        tmp_pos = new_pos;

                        dead_end = false;
                        last_dir = offset;
                        momentum -= 1.0; // Decrease momentum TODO: do this better lul
                        break;
                    };
                    if let Some(transition) = transitions.iter().find(|t| t.condition == collider.kind).cloned() {
                        update.updated = true;
                        update.new_pos = Some(new_pos);
                        update.transition = Some(transition);
                        update.new_momentum = 0.0;
                        return update;
                    } else if collider.kind.is_liquid() && !self.kind.is_liquid() {
                        update.updated = true;
                        update.new_pos = Some(new_pos);
                        update.swapped = true;
                        update.new_momentum = 0.0;
                        return update;
                    }
                }
                if !dead_end {
                    break;
                }
            }
            if dead_end {
                update.new_momentum = 0.0;
                break;
            }
        }

        update
    }
}
