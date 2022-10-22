use bevy::{
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};
use rand::random;

use crate::{greachers::gen::GreacherPart, util::rand_range_f32};

use super::{gen::{generate_greacher_head_texture, GreacherInfo}, behavior::{animate_greacher_body, GreacherBodyAnimation}};

struct GreetTimer(Timer);

pub struct GreacherHeadImageTemplate(pub Image);

pub struct GreacherGamePlugin;

impl Plugin for GreacherGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .insert_resource(GreacherHeadImageTemplate(Image {
                texture_descriptor: TextureDescriptor {
                    label: None,
                    size: Extent3d {
                        width: 10,
                        height: 10,
                        ..Default::default()
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::Rgba8Unorm,
                    usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
                },
                ..Default::default()
            }))
            .add_startup_system(setup)
            .add_system(generate_greacher_head)
            .add_system(animate_greacher_body);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    head_template: Res<GreacherHeadImageTemplate>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    for _ in 0..10 {
        create_new_greacher(
            &mut commands,
            &asset_server,
            &mut images,
            &mut texture_atlases,
            &head_template,
            Vec3::new(rand_range_f32(-50., 50.), rand_range_f32(-50., 50.), 0.),
        );
    }
}

fn generate_greacher_head(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    mut query: Query<(&mut GreacherInfo, &mut Handle<Image>)>,
    mut images: ResMut<Assets<Image>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (mut greacher_info, img_handle) in query.iter_mut() {
            println!("Generated GREACHER");
            let img = images.get_mut(&img_handle).expect("WHAT HTEH HELL");

            generate_greacher_head_texture(random(), img);

            greacher_info.mark_as_generated(GreacherPart::Head);
        }
    }
}

fn create_new_greacher(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    images: &mut ResMut<Assets<Image>>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    head_template: &GreacherHeadImageTemplate,
    position: Vec3,
) {
    let mut tex = head_template.0.clone();

    generate_greacher_head_texture(0, &mut tex);

    let handle = images.add(tex);

    let parent = commands
        .spawn_bundle(SpriteBundle {
            texture: handle,
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .insert(GreacherInfo::new())
        .id();

    let texture_handle = asset_server.load("indexed/legs.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 6.0), 8, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let child = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(0., -7., 1.),
            ..Default::default()
        })
        .insert(GreacherBodyAnimation(Timer::from_seconds(0.125, true)))
        .id();

    commands.entity(parent).push_children(&[child]);
}
