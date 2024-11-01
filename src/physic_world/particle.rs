use bevy::{color::Color, math::VectorSpace, prelude::*, sprite::Anchor};
use rand::prelude::*;

use crate::{
    io::ColorPalette,
    utils::{color, vec::VecParse},
};

const SLEEP_THRESHOLD: u32 = 25;
const PARTICLE_SIZE: Vec2 = Vec2::new(1.0, 1.0);
const SAND_MOVEMENT: ParticleMovement = &[
    MovementOptionGroup(&[IVec2::new(0, -1)]),
    MovementOptionGroup(&[IVec2::new(1, -1), IVec2::new(-1, -1)]),
];
const WATER_MOVEMENT: ParticleMovement = &[
    MovementOptionGroup(&[IVec2::new(0, -1)]),
    MovementOptionGroup(&[IVec2::new(1, -1), IVec2::new(-1, -1)]),
    MovementOptionGroup(&[IVec2::new(1, 0), IVec2::new(-1, 0)]),
];

pub struct MovementOptionGroup(&'static [IVec2]);

pub type ParticleMovement = &'static [MovementOptionGroup];

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleType {
    Sand,
    Water,
}

#[derive(Component, Debug, Clone)]
pub struct ParticleData {
    velocity: Vec2,
    sleep_counter: u32,
}

impl MovementOptionGroup {
    pub fn shuffled(&self) -> Vec<IVec2> {
        let mut shuffled = self.0.to_vec();
        shuffled.shuffle(&mut thread_rng());
        shuffled
    }
}

impl ParticleType {
    pub fn create(
        self,
        commands: &mut Commands,
        color_palette: &Res<ColorPalette>,
        position: IVec2,
        alpha: f32,
    ) -> Entity {
        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: self.color(color_palette).unwrap().with_alpha(alpha),
                        custom_size: Some(PARTICLE_SIZE),
                        anchor: Anchor::BottomLeft,
                        ..default()
                    },
                    transform: Transform::from_translation(position.as_vec3()),
                    ..default()
                },
                self,
                ParticleData::default(),
            ))
            .id()
    }

    pub fn color(&self, color_palette: &Res<ColorPalette>) -> Option<Color> {
        let start = std::time::SystemTime::now();
        let now = start.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let palette = color_palette.particle_color(self);
        let mut index = (now % palette.len() as u64) as usize;

        // add some randomness to the color
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.15) {
            index = (index + rng.gen_range(0..palette.len())) % palette.len();
        }

        Some(color::hex_to_color(&palette[index]).expect("Failed to parse color for particle"))
    }

    pub fn movement(&self) -> ParticleMovement {
        match self {
            ParticleType::Sand => SAND_MOVEMENT,
            ParticleType::Water => WATER_MOVEMENT,
        }
    }

    pub fn next_position<F>(&self, position: &IVec2, data: &mut ParticleData, lookup: F) -> Option<IVec2>
    where
        F: Fn(&IVec2) -> Option<Entity>,
    {
        let mut changed = false;
        let mut updated = *position;

        let velocity = data.ivec2_velocity();
        let mut y_velocity = velocity.y.abs();
        let mut last_dir = IVec2::ZERO;

        while y_velocity > 0 {
            // This might be an infinite loop in later versions
            let mut dead_end = true;
            for group in self.movement() {
                let mut shuffled = group.shuffled();
                shuffled.insert(0, last_dir);
                for direction in shuffled {
                    let next = updated + direction;
                    if lookup(&next).is_none() && next.y >= 0 {
                        changed = true;
                        dead_end = false;
                        updated = next;
                        y_velocity -= 1;
                        last_dir = direction;

                        break;
                    }
                }
                if !dead_end {
                    break;
                }
            }
            if dead_end {
                data.velocity.y = 0.0;
                break;
            }
        }

        if changed {
            Some(updated)
        } else {
            None
        }
    }
}

impl ParticleData {
    pub fn asleep(&self) -> bool {
        self.sleep_counter == 0
    }

    pub fn sleep(&mut self) {
        self.sleep_counter = self.sleep_counter.saturating_sub(1);
        // if self.sleep_counter == 0 {
        //     self.velocity = Vec2::ZERO;
        // }
    }

    pub fn wake(&mut self) {
        self.sleep_counter = SLEEP_THRESHOLD;
    }

    pub fn accelerate(&mut self, amount: Vec2) {
        self.velocity += amount;
    }

    pub fn ivec2_velocity(&self) -> IVec2 {
        self.velocity.as_ivec2()
    }
}

impl Default for ParticleData {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            sleep_counter: SLEEP_THRESHOLD,
        }
    }
}
