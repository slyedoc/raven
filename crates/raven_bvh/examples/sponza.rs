mod helpers;
use helpers::*;

use crate::helpers::camera_free::CameraFree;
use bevy::prelude::*;
use raven_bvh::prelude::*;

/// This is by far the worse performing example, far more triangles than the others

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelperPlugin, BvhPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // add tlas for camera
    let tlas = commands.spawn((Name::new("Tlas"), Tlas::default())).id();

    // camera
    commands.spawn((
        Name::new("Main Camera"),
        CameraFree, // Helper to move the camera around with WASD and mouse look with right mouse button
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        TlasCamera::new(128, 256, tlas),
    ));

    // light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Name::new("Sponza"),
        Transform::from_xyz(0.0, 1.0, 0.0),
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/sponza/sponza.gltf")),
        ),
        // This marker tells the BVH system to build nested children
        // for this entity, waits till asset is loaded
        SpawnSceneBvhForTlas(tlas),
    ));
}
