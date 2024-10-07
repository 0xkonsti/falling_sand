use bevy::prelude::*;

type ParticleBrush = &'static [IVec2];

const DOT_BRUSH: ParticleBrush = &[IVec2::new(0, 0)];

const LINE_BRUSH_SMALL: ParticleBrush = &[IVec2::new(-1, 0), IVec2::new(0, 0), IVec2::new(1, 0)];

const LINE_BRUSH_MEDIUM: ParticleBrush = &[
    IVec2::new(-2, 0),
    IVec2::new(-1, 0),
    IVec2::new(0, 0),
    IVec2::new(1, 0),
    IVec2::new(2, 0),
];

const LINE_BRUSH_LARGE: ParticleBrush = &[
    IVec2::new(-3, 0),
    IVec2::new(-2, 0),
    IVec2::new(-1, 0),
    IVec2::new(0, 0),
    IVec2::new(1, 0),
    IVec2::new(2, 0),
    IVec2::new(3, 0),
];

const CIRCLE_BRUSH_SMALL: ParticleBrush = &[
    IVec2::new(0, -1),
    IVec2::new(-1, 0),
    IVec2::new(0, 0),
    IVec2::new(1, 0),
    IVec2::new(0, 1),
];

const CIRCLE_BRUSH_MEDIUM: ParticleBrush = &[
    IVec2::new(0, -2),
    IVec2::new(-1, -1),
    IVec2::new(0, -1),
    IVec2::new(1, -1),
    IVec2::new(-2, 0),
    IVec2::new(-1, 0),
    IVec2::new(0, 0),
    IVec2::new(1, 0),
    IVec2::new(2, 0),
    IVec2::new(-1, 1),
    IVec2::new(0, 1),
    IVec2::new(1, 1),
    IVec2::new(0, 2),
];

const CIRCLE_BRUSH_LARGE: ParticleBrush = &[
    IVec2::new(0, -3),
    IVec2::new(-1, -2),
    IVec2::new(0, -2),
    IVec2::new(1, -2),
    IVec2::new(-2, -1),
    IVec2::new(-1, -1),
    IVec2::new(0, -1),
    IVec2::new(1, -1),
    IVec2::new(2, -1),
    IVec2::new(-3, 0),
    IVec2::new(-2, 0),
    IVec2::new(-1, 0),
    IVec2::new(0, 0),
    IVec2::new(1, 0),
    IVec2::new(2, 0),
    IVec2::new(3, 0),
    IVec2::new(-2, 1),
    IVec2::new(-1, 1),
    IVec2::new(0, 1),
    IVec2::new(1, 1),
    IVec2::new(2, 1),
    IVec2::new(-1, 2),
    IVec2::new(0, 2),
    IVec2::new(1, 2),
    IVec2::new(0, 3),
];

#[derive(Resource)]
pub struct Brush {
    current: usize,
    brushes: Vec<ParticleBrush>,
}

impl Brush {
    pub fn current(&self) -> ParticleBrush {
        self.brushes[self.current]
    }

    pub fn next(&mut self) {
        self.current = (self.current + 1) % self.brushes.len();
    }

    pub fn previous(&mut self) {
        self.current = (self.current + self.brushes.len() - 1) % self.brushes.len();
    }
}

impl Default for Brush {
    fn default() -> Self {
        Self {
            current: 5,
            brushes: vec![
                DOT_BRUSH,
                LINE_BRUSH_SMALL,
                LINE_BRUSH_MEDIUM,
                LINE_BRUSH_LARGE,
                CIRCLE_BRUSH_SMALL,
                CIRCLE_BRUSH_MEDIUM,
                CIRCLE_BRUSH_LARGE,
            ],
        }
    }
}
