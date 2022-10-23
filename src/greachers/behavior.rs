use bevy::prelude::*;

use super::{
    components::{
        Greacher, GreacherAnimationState, GreacherBodyAnimation, GreacherBodyType, LegState,
    },
    game_plugin::WorldMouse,
};

pub fn animate_greacher_body(
    time: Res<Time>,
    mut query_child: Query<(&Parent, &mut GreacherBodyAnimation, &mut TextureAtlasSprite)>,
    query_parent: Query<&Greacher>,
) {
    for (parent, mut animation, mut sprite) in &mut query_child {
        let greacher = query_parent.get(parent.get()).unwrap();

        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            let old_state = animation.state.clone();
            animation.state = get_greacher_state(greacher);

            if animation.state != old_state {
                animation.frame_counter = 0;
            } else {
                animation.frame_counter += 1;
            }

            match animation.state {
                GreacherAnimationState::Legs(state) => {
                    if greacher.velocity.x > 0.1 {
                        sprite.flip_x = false;
                    } else if greacher.velocity.x < -0.1 {
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

fn get_greacher_state(greacher: &Greacher) -> GreacherAnimationState {
    match greacher.body_type {
        GreacherBodyType::Legs => {
            if greacher.velocity.length() > 1. {
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
    mut greachers: Query<(&mut Greacher, &Transform)>,
) {
    for (mut greacher, transform) in &mut greachers {
        greacher.velocity += (**world_mouse
            - Vec2::new(transform.translation.x, transform.translation.y))
        .normalize_or_zero()
            * 64.
            * time.delta_seconds();
    }
}

pub fn handle_greacher_velocity(
    time: Res<Time>,
    mut greachers: Query<(&mut Greacher, &mut Transform)>,
) {
    for (mut greacher, mut transform) in &mut greachers {
        greacher.velocity = greacher.velocity.clamp_length_max(64.);
        transform.translation.x += greacher.velocity.x * time.delta_seconds();
        transform.translation.y += greacher.velocity.y * time.delta_seconds();
    }
}
