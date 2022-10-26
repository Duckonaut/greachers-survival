use super::components::*;
use super::systems::*;
use bevy::{prelude::*, render::camera::RenderTarget};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PostUpdate, broad_collision_pass);
    }
}
