use avian3d::prelude::*;
use bevy::prelude::*;
use raven_editor::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            EditorPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("MainCamera"),
        IsEditorCamera, // Used to enable picking
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.0, -0.5, 0.0)),
    ));

    // ground
    commands.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::default(),
        Collider::cuboid(10.0, 0.2, 10.0),
        RigidBody::Static,
    ));

    // cube
    commands.spawn((
        Name::new("Cube"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.3, 0.3))),
        Transform::from_xyz(0.0, 3.0, 0.0),
        Collider::cuboid(0.5, 0.5, 0.5),
        RigidBody::Dynamic,
    ));
}
