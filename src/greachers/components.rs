use bevy::prelude::*;
use bitmask_enum::bitmask;
use rand::{random, rngs::SmallRng, SeedableRng};

use super::gen::{generate_greacher_head_texture, generate_greacher_name};

#[derive(Component)]
pub struct Greacher {
    pub seed: u64,
    pub name: String,
    pub generated: GreacherParts,
    pub body_type: GreacherBodyType,
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
    pub const SIZE: f32 = 12.0;
    pub const STILL_EPSILON: f32 = 1.;

    pub fn new(head_texture: &mut Image) -> Greacher {
        let generated_flags = GreacherParts::none();

        let mut greacher = Greacher {
            seed: random(),
            name: String::new(),
            generated: generated_flags,
            body_type: GreacherBodyType::Legs,
        };

        greacher.generate(head_texture);

        greacher
    }

    pub fn mark_as_generated(&mut self, category: GreacherParts) {
        self.generated |= category;
    }

    pub fn is_part_generated(&self, category: GreacherParts) -> bool {
        self.generated.contains(category)
    }

    pub fn generate(&mut self, head_texture: &mut Image) {
        let mut rng = SmallRng::seed_from_u64(self.seed);

        self.name = generate_greacher_name(&mut rng);
        generate_greacher_head_texture(&mut rng, head_texture);
        self.mark_as_generated(GreacherParts::Head);
    }

    pub fn regenerate(&mut self, head_texture: &mut Image) {
        self.seed = random();

        self.generated = GreacherParts::none();

        self.generate(head_texture);
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

#[derive(Component, Default)]
pub struct MovementHistory {
    last_position: Vec2,
    pub actually_moved: Vec2,
}

impl MovementHistory {
    pub fn set_last_position(mut query: Query<(&Transform, &mut MovementHistory)>) {
        for (transform, mut history) in &mut query {
            history.last_position = transform.translation.truncate();
        }
    }

    pub fn set_actually_moved(mut query: Query<(&Transform, &mut MovementHistory)>) {
        for (transform, mut history) in &mut query {
            history.actually_moved = transform.translation.truncate() - history.last_position;
        }
    }
}

#[derive(Component)]
pub struct RadiusCollider {
    pub radius: f32,
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct Velocity {
    pub inner: Vec2,
}
