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

pub const EMPTY_SPATIAL_PARTITIONS: [Option<(u8, u8)>; 4] = [None; 4];
pub const PARTITIONS_HORIZONTAL: u8 = 8;
pub const PARTITIONS_VERTICAL: u8 = 8;

#[derive(Component)]
pub struct SimpleCollider {
    pub spatial_partitions: [Option<(u8, u8)>; 4],
    pub shape: ColliderShape,
}

impl SimpleCollider {
    pub fn new(shape: ColliderShape) -> Self {
        SimpleCollider {
            spatial_partitions: EMPTY_SPATIAL_PARTITIONS,
            shape,
        }
    }
}

pub enum ColliderShape {
    Circle(f32),
}

impl ColliderShape {
    pub fn get_corners(&self, transform: &Transform) -> [Vec2; 4] {
        match self {
            ColliderShape::Circle(radius) => {
                let scaled_radius_x = radius * transform.scale.x;
                let scaled_radius_y = radius * transform.scale.y;

                [
                    Vec2::new(
                        transform.translation.x - scaled_radius_x,
                        transform.translation.y - scaled_radius_y,
                    ),
                    Vec2::new(
                        transform.translation.x + scaled_radius_x,
                        transform.translation.y - scaled_radius_y,
                    ),
                    Vec2::new(
                        transform.translation.x - scaled_radius_x,
                        transform.translation.y + scaled_radius_y,
                    ),
                    Vec2::new(
                        transform.translation.x + scaled_radius_x,
                        transform.translation.y + scaled_radius_y,
                    ),
                ]
            }
        }
    }
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct Velocity {
    pub inner: Vec2,
}
