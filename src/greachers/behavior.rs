use std::time::Duration;

use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::basics::components::{MovementHistory, Velocity};

use super::{
    components::{
        Greacher, GreacherAnimationState, GreacherBodyAnimation, GreacherBodyType, LegState,
    },
    game_plugin::{WorldMouse, MIN_Y_COORD},
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
                    if velocity.x > 0.1 {
                        sprite.flip_x = false;
                    } else if velocity.x < -0.1 {
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
        velocity.inner += (**world_mouse
            - Vec2::new(transform.translation.x, transform.translation.y))
        .normalize_or_zero()
            * 72.
            * time.delta_seconds();
    }
}

pub fn apply_velocity(time: Res<Time>, mut entities: Query<(&mut Velocity, &mut Transform)>) {
    for (mut velocity, mut transform) in &mut entities {
        velocity.inner = velocity.clamp_length_max(24.);
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
        transform.translation.z = -transform.translation.y - MIN_Y_COORD;
    }
}

pub fn handle_greacher_collisions(
    time: Res<Time>,
    mut set: ParamSet<(
        Query<(Entity, &Transform), With<Greacher>>,
        Query<(Entity, &mut Transform), With<Greacher>>,
    )>,
) {
    let greachers_push = set.p0();

    let mut pushes = HashMap::<Entity, Vec2>::with_capacity(greachers_push.iter().count());

    for (entity_pushing, transform_pushing) in &greachers_push {
        for (entity_pushed, transform_pushed) in &greachers_push {
            if entity_pushing == entity_pushed {
                continue;
            }

            let displacement = Vec2::new(
                transform_pushed.translation.x - transform_pushing.translation.x,
                transform_pushed.translation.y - transform_pushing.translation.y,
            );

            let displacement = if displacement.length() < Greacher::SIZE {
                let dis = displacement;

                if let Some(vec) = displacement.try_normalize() {
                    vec * (Greacher::SIZE - dis.length())
                } else {
                    Vec2::Y * Greacher::SIZE
                }
            } else {
                Vec2::ZERO
            };

            match pushes.get(&entity_pushed) {
                Some(vec) => {
                    pushes.insert(entity_pushed, displacement + *vec);
                }
                None => {
                    pushes.insert(entity_pushed, displacement);
                }
            }
        }
    }

    let mut greachers_get_pushed = set.p1();

    for (entity, mut transform_getting_pushed) in greachers_get_pushed.iter_mut() {
        if let Some(displacement) = pushes.get(&entity) {
            transform_getting_pushed.translation.x += displacement.x * time.delta_seconds() * 8.;
            transform_getting_pushed.translation.y += displacement.y * time.delta_seconds() * 8.;
        }
    }
}
