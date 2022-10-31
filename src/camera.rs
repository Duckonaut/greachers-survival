use std::f32::consts::PI;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{shape::Plane, *},
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        render_resource::{
            AsBindGroup, Extent3d, SamplerDescriptor, TextureDescriptor, TextureDimension,
            TextureFormat, TextureUsages,
        },
        texture::ImageSampler,
        view::RenderLayers,
    },
};

#[derive(Component)]
pub struct GameCamera;

pub struct GameWorldRenderLayer(pub RenderLayers);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameWorldRenderLayer(RenderLayers::layer(1)))
            .add_plugin(MaterialPlugin::<OutlineMaterial>::default())
            .add_startup_system(setup_dpass)
            .add_startup_system(setup_msaa);
    }
}

pub fn create_render_texture(size: Extent3d) -> Image {
    Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        sampler_descriptor: ImageSampler::Descriptor(SamplerDescriptor {
            mag_filter: bevy::render::render_resource::FilterMode::Nearest,
            min_filter: bevy::render::render_resource::FilterMode::Nearest,
            ..default()
        }),
        ..default()
    }
}

fn setup_dpass(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<OutlineMaterial>>,
    mut images: ResMut<Assets<Image>>,
    game_world_render_layer: Res<GameWorldRenderLayer>,
) {
    let size = Extent3d {
        width: 320,
        height: 180,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = create_render_texture(size);

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    let material_handle = materials.add(OutlineMaterial {
        render_texture: image_handle.clone(),
    });

    let plane_handle = meshes.add(Mesh::from(Plane { size: 1.0 }));

    commands.spawn_bundle(PbrBundle {
        mesh: plane_handle.clone(),
        transform: Transform::from_rotation(
            Quat::from_rotation_z(PI) * Quat::from_rotation_x(PI / 2.0),
        )
        .with_scale(Vec3::new(320.0, 1.0, 180.0))
        .with_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    });

    commands.spawn_bundle(MaterialMeshBundle {
        material: material_handle,
        mesh: plane_handle,
        transform: Transform::from_rotation(
            Quat::from_rotation_z(PI) * Quat::from_rotation_x(PI / 2.0),
        )
        .with_scale(Vec3::new(320.0, 1.0, 180.0))
        .with_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..default()
    });

    commands
        .spawn_bundle(Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::Rgba {
                    red: 0.,
                    green: 0.,
                    blue: 0.,
                    alpha: 0.,
                }),
            },
            camera: Camera {
                // render before the "main pass" camera
                priority: -1,
                target: RenderTarget::Image(image_handle),
                ..default()
            },
            ..default()
        })
        .insert(game_world_render_layer.0)
        .insert(GameCamera);

    commands.spawn_bundle(Camera3dBundle {
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::GREEN),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 2.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        projection: bevy::render::camera::Projection::Orthographic(OrthographicProjection {
            left: -160.0,
            right: 160.0,
            bottom: -90.0,
            top: 90.0,
            scaling_mode: bevy::render::camera::ScalingMode::None,
            ..default()
        }),
        ..Default::default()
    });
}

fn setup_msaa(mut msaa: ResMut<Msaa>) {
    msaa.samples = 1;
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Component)]
#[uuid = "1e55b055-f4c4-c1c2-d1d2-d3d4d5d6d7d8"]
pub struct OutlineMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub render_texture: Handle<Image>,
}

impl Material for OutlineMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/outline.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
