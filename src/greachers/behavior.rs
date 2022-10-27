use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::basics::components::MovementHistory;

use super::{
    components::{
        Greacher, GreacherAnimationState, GreacherBodyAnimation, GreacherBodyType, LegState,
    },
    game_plugin::WorldMouse,
};

pub fn animate_greacher_body(
    time: Res<Time>,
    mut query_child: Query<(&Parent, &mut GreacherBodyAnimation, &mut TextureAtlasSprite)>,
    query_parent: Query<(&Greacher, &Velocity, &MovementHistory)>,
) {
    for (parent, mut animation, mut sprite) in &mut query_child {
        let (greacher, velocity, movement_history) = query_parent.get(parent.get()).unwrap();

        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            let delta = movement_history.actually_moved.length() / time.delta_seconds();
            if delta > 4. {
                animation
                    .timer
                    .set_duration(Duration::from_secs_f32(1. / 16.));
            } else {
                animation
                    .timer
                    .set_duration(Duration::from_secs_f32(1. / (8. + (2. * delta))));
            }

            let old_state = animation.state.clone();
            animation.state = get_greacher_state(&time, greacher, movement_history);

            if animation.state != old_state {
                animation.frame_counter = 0;
            } else {
                animation.frame_counter += 1;
            }

            match animation.state {
                GreacherAnimationState::Legs(state) => {
                    if velocity.linvel.x > 0.1 {
                        sprite.flip_x = false;
                    } else if velocity.linvel.x < -0.1 {
                        sprite.flip_x = true;
                    }

                    match state {
                        LegState::Idle => {
                            if animation.frame_counter >= 6 {
                                animation.frame_counter = 0;
                            }

                            sprite.index = animation.frame_counter;
                        }
                        LegState::Run => {
                            if animation.frame_counter >= 8 {
                                animation.frame_counter = 0;
                            }

                            sprite.index = 8 + animation.frame_counter;
                        }
                    }
                }
                GreacherAnimationState::Wings => {
                    if animation.frame_counter >= 5 {
                        animation.frame_counter = 0;
                    }

                    sprite.index = animation.frame_counter
                }
            }
        }
    }
}

fn get_greacher_state(
    time: &Time,
    greacher: &Greacher,
    movement_history: &MovementHistory,
) -> GreacherAnimationState {
    match greacher.body_type {
        GreacherBodyType::Legs => {
            if movement_history.actually_moved.length() / time.delta_seconds()
                > Greacher::STILL_EPSILON
            {
                GreacherAnimationState::Legs(LegState::Run)
            } else {
                GreacherAnimationState::Legs(LegState::Idle)
            }
        }
        GreacherBodyType::Wings => GreacherAnimationState::Wings,
    }
}

pub fn go_towards_mouse(
    time: Res<Time>,
    world_mouse: Res<WorldMouse>,
    mut greachers: Query<(&mut Velocity, &Transform), With<Greacher>>,
) {
    for (mut velocity, transform) in &mut greachers {
        velocity.linvel += (**world_mouse
            - Vec2::new(transform.translation.x, transform.translation.y))
        .normalize_or_zero()
            * 72.
            * time.delta_seconds();
    }
}

pub fn set_z(mut entities: Query<&mut Transform, With<Greacher>>) {
    let min_y = entities
        .iter()
        .map(|t| t.translation.y)
        .reduce(f32::min)
        .unwrap();
    let max_y = entities
        .iter()
        .map(|t| t.translation.y)
        .reduce(f32::max)
        .unwrap();

    let spread = max_y - min_y;

    for mut transform in &mut entities {
        transform.translation.z = 1. + ((-transform.translation.y + min_y) / spread);
    }
}

pub fn limit_greacher_velocity(mut entities: Query<&mut Velocity>) {
    for mut velocity in &mut entities {
        velocity.linvel = velocity.linvel.clamp_length_max(16.);
    }
}
