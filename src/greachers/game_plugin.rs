use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
};

use bevy_rapier2d::prelude::*;

use crate::{basics::components::MovementHistory, camera::GameCamera, util::rand_range_f32};

use super::{
    behavior::{animate_greacher_body, go_towards_mouse, limit_greacher_velocity, set_z},
    components::{Greacher, GreacherBodyAnimation},
};

struct GreetTimer(Timer);

#[derive(Deref, DerefMut)]
pub struct WorldMouse(Vec2);

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
            .insert_resource(WorldMouse(Vec2::ZERO))
            .add_startup_system(setup)
            .add_system_to_stage(CoreStage::PreUpdate, world_cursor_pos)
            .add_system_to_stage(CoreStage::PreUpdate, MovementHistory::set_last_position)
            .add_system_to_stage(CoreStage::PostUpdate, MovementHistory::set_actually_moved)
            //.add_system(periodically_regenerate_greachers)
            .add_system(go_towards_mouse)
            .add_system(set_z)
            .add_system(limit_greacher_velocity)
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
    commands
        .spawn_bundle(Camera2dBundle {
            ..Default::default()
        })
        .insert(GameCamera);

    for _ in 0..1000 {
        create_new_greacher(
            &mut commands,
            &asset_server,
            &mut images,
            &mut texture_atlases,
            &head_template,
            Vec2::new(rand_range_f32(-100., 100.), rand_range_f32(-100., 100.)),
        );
    }
}

fn create_new_greacher(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    images: &mut ResMut<Assets<Image>>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    head_template: &GreacherHeadImageTemplate,
    position: Vec2,
) {
    let mut tex = head_template.0.clone();

    let greacher = Greacher::new(&mut tex);

    let greacher_body_type = greacher.body_type;

    let handle = images.add(tex);

    let parent = commands
        .spawn_bundle(SpriteBundle {
            texture: handle,
            transform: Transform::from_translation(Vec3::new(
                position.x,
                position.y,
                (greacher.seed % 1_000_000) as f32 / 1_000_000.,
            )),
            ..Default::default()
        })
        .insert(greacher)
        .insert(MovementHistory::default())
        .insert(Velocity::default())
        .insert(Collider::ball(5.))
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(0.))
        .insert(Damping {
            linear_damping: 0.9,
            angular_damping: 1.0,
        })
        .insert(LockedAxes::ROTATION_LOCKED)
        .id();

    let texture_handle = asset_server.load("indexed/legs.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 6.0), 8, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let child = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(0., -7., 0.),
            ..Default::default()
        })
        .insert(GreacherBodyAnimation::new(&greacher_body_type))
        .id();

    commands.entity(parent).push_children(&[child]);
}

fn world_cursor_pos(
    wnds: Res<Windows>,
    mut world_mouse: ResMut<WorldMouse>,
    camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        world_mouse.0 = world_pos;
    }
}
