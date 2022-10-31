use bevy::{
    prelude::*,
    render::{
        render_resource::{AddressMode, FilterMode, SamplerDescriptor},
        texture::ImageSettings,
    },
    window::WindowMode,
};
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use camera::CameraPlugin;
use color::{IndexerPlugin, GreacherPalettes};
use fps_counter::FpsCounterPlugin;
use greachers::game_plugin::GreacherGamePlugin;
use states::AppState;

mod basics;
mod camera;
mod color;
mod fps_counter;
mod greachers;
mod states;
mod util;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 320.0,
            height: 180.0,
            scale_factor_override: Some(4.0),
            title: "Greacher Survival".to_string(),
            resizable: false,
            cursor_visible: true,
            cursor_locked: false,
            mode: WindowMode::Windowed,
            present_mode: bevy::window::PresentMode::AutoNoVsync,
            ..Default::default()
        })
        .insert_resource(ImageSettings {
            default_sampler: SamplerDescriptor {
                address_mode_u: AddressMode::ClampToEdge,
                address_mode_v: AddressMode::ClampToEdge,
                mag_filter: FilterMode::Nearest,
                min_filter: FilterMode::Nearest,
                mipmap_filter: FilterMode::Nearest,
                ..Default::default()
            },
        })
        .insert_resource(GreacherPalettes::default())
        .add_plugins(DefaultPlugins)
        .add_state(AppState::InGame)
        .add_plugin(FpsCounterPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
        .add_plugin(GreacherGamePlugin)
        .run();
}
