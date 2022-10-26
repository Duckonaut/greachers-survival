use basics::plugins::CollisionPlugin;
use bevy::{
    prelude::*,
    render::{
        render_resource::{AddressMode, FilterMode, SamplerDescriptor},
        texture::ImageSettings,
    },
    window::WindowMode,
};
use fps_counter::FpsCounterPlugin;
use greachers::game_plugin::GreacherGamePlugin;

mod color;
mod fps_counter;
mod greachers;
mod basics;
mod util;
mod camera;

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
        .add_plugins(DefaultPlugins)
        .add_plugin(FpsCounterPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(GreacherGamePlugin)
        .run();
}
