use bevy::prelude::*;

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
