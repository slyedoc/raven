use bevy::{
    core_pipeline::{bloom::Bloom},
    pbr::{light_consts::lux, Atmosphere, CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
    render::camera::Exposure,
};
use avian3d::prelude::*;
use raven_editor::prelude::*;
use raven_terrain::prelude::*;
use raven_util::prelude::*;

fn main() -> AppExit {
    App::new()
        //.insert_resource(DefaultOpaqueRendererMethod::deferred())
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Raven Terrain".into(),
                    resolution: (1400., 1000.).into(),
                    ..default()
                }),
                ..default()
            }),
            TerrainPlugin,
            //WaterPlugin,
            PhysicsPlugins::default(),
            EditorPlugin::default(),
            CameraFreePlugin,
        ))
        .add_systems(Startup, setup)
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .run()
}

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
        Transform::from_xyz(0., 200.0, -300.),
        Msaa::Off,        
        Atmosphere::EARTH,
        Exposure::SUNLIGHT,
        //Tonemapping::AcesFitted,
        Bloom::NATURAL,
        CameraFree, // Camera Controller
        // EnvironmentMapLight {
        //      diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
        //      specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        //      intensity: 5000.0,
        //      ..default()
        //  },
        // Skybox {
        //      image: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        //      brightness: 5000.0,
        //      ..default()
        // },
        // ScreenSpaceReflections::default(),
        // Fxaa::default(),
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: lux::RAW_SUNLIGHT,
            ..default()
        },
        CascadeShadowConfigBuilder::default().build(),
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
        RigidBody::Dynamic,
        Transform::from_xyz(0.0, 20.0, 0.0),
    ));

    // commands.spawn((
    //     Name::new("Terrain"),
    //     TerrainChunk {
            
    //         ..default()
    //     },
    //     Transform::from_xyz(0.0, 0.0, 0.0),
    // ));

    commands.spawn((
        Name::new("Terrain Gen"),
        TerrainGenerator {            
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // commands.spawn((
    //     Name::new("Water"),
    //     WaterGenerator,
    //     Transform::from_scale(Vec3::splat(1000.0)),
    // ));
    
    commands.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, vec2(50.0, 50.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 1.0, 1.0),            
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Collider::cuboid(50.0, 0.01, 50.0),
        RigidBody::Static,
    ));
}
