use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    pbr::{Atmosphere, DirectionalLightShadowMap, light_consts::lux},
    prelude::*,
    render::camera::Exposure,
};
use avian3d::prelude::*;
use raven_editor::prelude::*;
use raven_brain::prelude::*;
use raven_util::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Raven Terrain".into(),
                    resolution: (1400., 1000.).into(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            BrainPlugin,
            EditorPlugin::default(),
            CameraFreePlugin,
        ))
        .add_systems(Startup, setup)
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .run()
}

/// Sets up the camera with orbital controls
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Transform::from_xyz(0., 2., -10.).looking_at(Vec3::ZERO, Vec3::Y),
        Atmosphere::EARTH,
        Exposure::SUNLIGHT,
        Tonemapping::AcesFitted,
        Bloom::NATURAL,
        CameraFree, // Camera Controller
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: lux::RAW_SUNLIGHT,
            ..default()
        },
        Transform::from_xyz(-1.0, 1.0, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Name::new("Cube"),
        Collider::cuboid(1.0, 1.0, 1.0),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            metallic: 0.5,
            perceptual_roughness: 0.5,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Ground
    commands.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, vec2(50.0, 50.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            metallic: 0.0,
            perceptual_roughness: 0.0,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        ColliderConstructor::TrimeshFromMesh,
        RigidBody::Static,
    ));
    
}
