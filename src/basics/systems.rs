use bevy::{prelude::*, render::camera::RenderTarget};

use crate::camera::GameCamera;

use super::components::*;

pub fn broad_collision_pass(
    windows: Res<Windows>,
    mut query: Query<(&mut SimpleCollider, &GlobalTransform)>,
    camera: Query<(&GlobalTransform, &Camera), With<GameCamera>>,
) {
    let (camera_transform, camera) = camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = if let RenderTarget::Window(id) = camera.target {
        match windows.get(id) {
            Some(v) => v,
            None => {
                println!("NO WINDOW, NO COLLISION PARTITIONING. L.");
                return;
            },
        }
    } else {
        windows.get_primary().unwrap()
    };

    let window_size = Vec2::new(window.width() as f32, window.height() as f32);

    let partition_size = Vec2::new(
        window_size.x / PARTITIONS_HORIZONTAL as f32,
        window_size.y / PARTITIONS_VERTICAL as f32,
    );

    let camera_translation = camera_transform.translation();

    let camera_left_bottom = camera_translation.truncate() - window_size / 2.;

    for (mut collider, transform) in &mut query {
        collider.spatial_partitions = EMPTY_SPATIAL_PARTITIONS;

        let corners = collider.shape.get_corners(&transform.compute_transform());

        let mut i = 0;

        for corner in corners {
            let screen_space_corner = corner - camera_left_bottom;

            let predicted_partition = (
                (screen_space_corner.x / partition_size.x).clamp(0., PARTITIONS_HORIZONTAL as f32)
                    as u8,
                (screen_space_corner.y / partition_size.y).clamp(0., PARTITIONS_VERTICAL as f32)
                    as u8,
            );

            if !collider
                .spatial_partitions
                .contains(&Some(predicted_partition))
            {
                collider.spatial_partitions[i] = Some(predicted_partition);
                i += 1;
            }
        }
    }
}
