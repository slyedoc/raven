use bevy::{color::palettes::tailwind, prelude::*};
use raven_util::prelude::*;
use bevy_simple_subsecond_system::prelude::*;

mod foo_material;
use foo_material::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Raven: Lab".to_string(),
                    ..default()
                }),
                ..default()
            }),
            SimpleSubsecondPlugin::default(),
            CameraFreePlugin,
            FooMaterialPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut foo_materials: ResMut<Assets<FooExtendedMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraFree,
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
    ));

    // Standard material plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.8, 0.8),
            ..default()
        })),
        Transform::from_xyz(-6.0, 0.0, 0.0),
    ));

    // FooMaterial plane that displays UV coordinates
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(foo_materials.add(FooExtendedMaterial {
            base: StandardMaterial {
                base_color: tailwind::AMBER_100.into(),
                ..default()
            },
            extension: FooMaterial {
                base_color: 1.0, // This will be used to display UV coordinates
            },
        })),
        Transform::from_xyz(6.0, 0.0, 0.0),
    ));
}