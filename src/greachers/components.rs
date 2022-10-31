use bevy::prelude::*;
use bitmask_enum::bitmask;
use rand::{random, rngs::SmallRng, SeedableRng, Rng};

use crate::{color::{GreacherColorPalette, GreacherPalettes}, util::SliceExt};

use super::gen::{generate_greacher_head_texture, generate_greacher_name};

#[derive(Component, Clone)]
pub struct Greacher {
    pub seed: u64,
    pub name: String,
    pub generated: GreacherParts,
    pub body_type: GreacherBodyType,
    pub palette: (usize, GreacherColorPalette),
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum GreacherBodyType {
    Legs,
    Wings,
}

#[bitmask(u8)]
pub enum GreacherParts {
    Head,
    Body,
    Stats,
    Name,
}

impl Greacher {
    pub const SIZE: f32 = 6.0;
    pub const STILL_EPSILON: f32 = 1.;

    pub fn new(head_texture: &mut Image, palettes: &GreacherPalettes) -> Greacher {
        let generated_flags = GreacherParts::none();

        let mut greacher = Greacher {
            seed: random(),
            name: String::new(),
            generated: generated_flags,
            body_type: GreacherBodyType::Legs,
            palette: (0, GreacherColorPalette::default())
        };

        greacher.generate(head_texture, palettes);

        greacher
    }

    pub fn mark_as_generated(&mut self, category: GreacherParts) {
        self.generated |= category;
    }

    pub fn is_part_generated(&self, category: GreacherParts) -> bool {
        self.generated.contains(category)
    }

    pub fn generate(&mut self, head_texture: &mut Image, palettes: &GreacherPalettes) {
        let mut rng = SmallRng::seed_from_u64(self.seed);

        self.name = generate_greacher_name(&mut rng);
        let palette_index = rng.gen_range(0..palettes.palettes.len());
        self.palette = (palette_index, palettes.palettes[palette_index].clone());
        generate_greacher_head_texture(&mut rng, head_texture, &self.palette.1);
        self.mark_as_generated(GreacherParts::Head);
    }

    pub fn regenerate(&mut self, head_texture: &mut Image, palettes: &GreacherPalettes) {
        self.seed = random();

        self.generated = GreacherParts::none();

        self.generate(head_texture, palettes);
    }
}

#[derive(Component)]
pub struct GreacherBodyAnimation {
    pub timer: Timer,
    pub frame_counter: usize,
    pub state: GreacherAnimationState,
}

#[derive(Clone, Eq, PartialEq)]
pub enum GreacherAnimationState {
    Legs(LegState),
    Wings,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum LegState {
    Idle,
    Run,
}

impl GreacherBodyAnimation {
    pub fn new(body_type: &GreacherBodyType) -> Self {
        GreacherBodyAnimation {
            timer: Timer::from_seconds(0.125, true),
            frame_counter: 0,
            state: match body_type {
                GreacherBodyType::Legs => GreacherAnimationState::Legs(LegState::Idle),
                GreacherBodyType::Wings => GreacherAnimationState::Wings,
            },
        }
    }
}
