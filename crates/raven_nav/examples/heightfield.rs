mod common; // helper functions
use common::*;

use avian3d::prelude::*;
use bevy::{color::palettes::tailwind, math::bounding::RayCast3d, prelude::*, window::WindowResolution};
use raven_nav::prelude::*;
use raven_bvh::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Raven: Simple".to_string(),
                    resolution: WindowResolution::new(1920.0, 1080.0),
                    ..default()
                }),
                ..default()
            }),
            // physics
            PhysicsPlugins::default(),
            NavPlugin,
            NavDebugPlugin::default(),
            ExampleCommonPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, ray_cast)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    agent_spawner: Res<AgentSpawner>,
) {

    // spawn default waymap for now
    commands.spawn((
        Name::new("Nav"),
        Nav::new(0.5, 1.9, Vec3::splat(300.0)),
        NavMovement, // helper to move the waymap around with Arrow Keys to see regeneration
    ));


    commands.spawn((
        Name::new("Camera"),
        //BvhCamera::new(512, 512), // debug camera for bvh
        CameraFree,
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Transform::from_xyz(0.0, 20.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Name::new("Directional Light"),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.0, -0.5, 0.0)),
    ));

    

    // heightfield
    let resolution = 250; // is this large?
    let oct = 10.0;
    let height_scale = 8.0;
    let scale = Vec3::new(resolution as f32, height_scale, resolution as f32);

    let mut heightfield = vec![vec![0.0; resolution]; resolution];
    for x in 0..resolution {
        for y in 0..resolution {
            heightfield[x][y] = (x as f32 / oct).sin() + (y as f32 / oct).cos();
        }
    }

    commands.spawn((
        Name::new("Heightfield"),
        Transform::from_xyz(0., -15.0, 0.),
        Mesh3d(meshes.add(generate_mesh_from_heightfield(&heightfield, scale, true))),
        MeshMaterial3d(materials.add(Color::srgb(0.7, 0.7, 0.8))),
        Collider::heightfield(heightfield, scale),
        RigidBody::Static,
        NavMeshAffector::default(), // Only entities with a NavMeshAffector component will contribute to the nav-mesh.
    ));

    commands.spawn((
        Name::new("Agent 1"),
        agent_spawner.spawn(),
        Agent,
        Transform::from_xyz(0.0, 2.0, 2.0),
    ));

    // commands.spawn((
    //     Name::new("Cube"),
    //     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    //     MeshMaterial3d(materials.add(Color::srgb(0.5, 0.3, 0.3))),
    //     Transform::from_xyz(0.0, 10.0, 0.0),
    //     Collider::cuboid(1.0, 1.0, 1.0),
    //     RigidBody::Dynamic,
    //     NavMeshAffector, // Only entities with a NavMeshAffector component will contribute to the nav-mesh.
    // ));

    commands.spawn((
        Name::new("Text"),
        Text::new("M: Toggle NavMesh Debug Draw"),
        TextFont {
            font_size: 10.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            right: Val::Px(5.0),
            ..default()
        },
    ));
}


fn ray_cast(    
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    tlas_query: Single<Entity, (With<Nav>, With<Tlas>)>,
    tlas: TlasCast,
    mut gizmos: Gizmos,
    mut nav_path: NavPath,
    input: Res<ButtonInput<MouseButton>>,
    mut start_pos: Local<Vec3>,
    mut end_pos: Local<Vec3>,
) {
    let (camera, camera_transform) = *camera_query;
    let tlas_entity = *tlas_query;
    
    // Use Right mouse buttons to set start 
    if input.pressed(MouseButton::Right) {        
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
        let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };

        let ray_cast = RayCast3d::from_ray(ray, f32::MAX);
        if let Some((_e, hit)) = tlas.intersect_tlas(&ray_cast, tlas_entity) {        
            *start_pos = ray.get_point(hit.distance);
        }
    }

    // Use Left mouse button to set end
    if input.pressed(MouseButton::Left) {        
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
        let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };
        let ray_cast = RayCast3d::from_ray(ray, f32::MAX);
        if let Some((_e, hit)) = tlas.intersect_tlas(&ray_cast, tlas_entity) {        
            *end_pos = ray.get_point(hit.distance);
        }
    }
    
    gizmos.sphere(*start_pos, 0.1, tailwind::GREEN_400);      
    gizmos.sphere(*end_pos, 0.1, tailwind::RED_400);                
    gizmos.line(*start_pos, *end_pos, tailwind::YELLOW_400);    

    // Run pathfinding to get a polygon path.
    match nav_path.find_path(tlas_entity, *start_pos, *end_pos, None, Some(&[1.0, 0.5])) {
        Ok(path) => gizmos.linestrip(path, tailwind::BLUE_300),
        Err(error) => error!("Error with pathfinding: {:?}", error),
    }    
}