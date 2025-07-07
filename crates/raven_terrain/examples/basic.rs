use avian3d::prelude::*;
use bevy::{
    core_pipeline::bloom::Bloom, ecs::identifier::error, image::{
        ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler,
        ImageSamplerDescriptor,
    }, pbr::{light_consts::lux, Atmosphere, CascadeShadowConfigBuilder, DirectionalLightShadowMap}, prelude::*, render::camera::Exposure
};
use bevy_asset_loader::prelude::*;
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
        .init_state::<MyStates>()
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::AssetProcessing)
                .load_collection::<TerrainAssets>(),
        )
        .add_systems(OnEnter(MyStates::AssetProcessing), fix_assets)
        .add_systems(Startup, setup)
        .add_systems(OnEnter(MyStates::Next), setup_with_assets)
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .run()
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
    #[default]
    AssetLoading,
    AssetProcessing,
    Next,
}

#[derive(AssetCollection, Resource)]
struct TerrainAssets {

    // // basis-universal
    // #[asset(path = "array_material/albedo.basis")]
    // base_color: Handle<Image>,
    // #[asset(path = "array_material/ao.basis")]
    // occlusion: Handle<Image>,
    // #[asset(path = "array_material/normal.basis")]
    // normal_map: Handle<Image>,
    // #[asset(path = "array_material/metal_rough.basis")]
    // metal_rough: Handle<Image>,


    // ktx2
    #[asset(path = "array_material/albedo.ktx2")]
    base_color: Handle<Image>,
    #[asset(path = "array_material/ao.ktx2")]
    occlusion: Handle<Image>,
    #[asset(path = "array_material/normal.ktx2")]
    normal_map: Handle<Image>,
    #[asset(path = "array_material/metal_rough.ktx2")]
    metal_rough: Handle<Image>,


    // png
    // #[asset(path = "textures/array_texture.png")]
    // base_color: Handle<Image>,
    // #[asset(path = "textures/array_texture.png")]
    // occlusion: Handle<Image>,
    // #[asset(path = "textures/array_texture.png")]
    // normal_map: Handle<Image>,
    // #[asset(path = "textures/array_texture.png")]
    // metal_rough: Handle<Image>,
}

fn fix_assets(
    handles: Res<TerrainAssets>,
    mut images: ResMut<Assets<Image>>,
    mut app_state: ResMut<NextState<MyStates>>,
) {    
    
    for handle in [
        &handles.base_color,
        // &handles.occlusion,
        // &handles.normal_map,
        // &handles.metal_rough,
    ] {
        let image = images.get_mut(handle).unwrap();

        // Create a new array texture asset from the loaded texture.
        // let array_layers = 4;
        // image.reinterpret_stacked_2d_as_array(array_layers);
    }
    



    app_state.set(MyStates::Next);
}

// fn repeat(settings: &mut ImageLoaderSettings) {
//     *settings = ImageLoaderSettings {
//         sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
//             address_mode_u: ImageAddressMode::Repeat,
//             address_mode_v: ImageAddressMode::Repeat,
//             address_mode_w: ImageAddressMode::Repeat,
//             mag_filter: ImageFilterMode::Linear,
//             min_filter: ImageFilterMode::Linear,
//             mipmap_filter: ImageFilterMode::Linear,
//             ..default()
//         }),
//         ..default()
//     }
// }

fn setup_with_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,    
    handles: Res<TerrainAssets>,
    mut tri_materials: ResMut<Assets<TriplanarMaterial>>,
    
) {    

    
    let mut sphere_mesh = Mesh::try_from(Sphere::new(5.0).mesh().ico(6).unwrap()).unwrap();

    let material_weights: Vec<u32> = sphere_mesh
        .attribute(Mesh::ATTRIBUTE_NORMAL)
        .unwrap()
        .as_float3()
        .unwrap()
        .iter()
        .map(|p| {
            let p = Vec3::from(*p);
            let w = sigmoid(signed_weight_to_unsigned(p.dot(Vec3::X)), 10.0);
            let w0 = (w * 255.0).clamp(0.0, 255.0) as u32;
            let w1 = 255 - w0;
            encode_weights([w0, 0, w1, 0])
            // encode_weights([255, 0, 0, 0])
        })
        .collect();
    sphere_mesh.insert_attribute(ATTRIBUTE_MATERIAL_WEIGHTS, material_weights);

    commands.spawn((
        Name::new("Triplane Sphere"),
        Mesh3d(meshes.add(sphere_mesh)),
        MeshMaterial3d(tri_materials.add(TriplanarMaterial {
            metallic: 0.05,
            perceptual_roughness: 0.9,

            base_color_texture: Some(handles.base_color.clone()),
            emissive_texture: None,
            metallic_roughness_texture: Some(handles.metal_rough.clone()),
            normal_map_texture: Some(handles.normal_map.clone()),
            occlusion_texture: Some(handles.occlusion.clone()),

            uv_scale: 1.0,
            ..default()
        })),
    ));
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
        TerrainGenerator { ..default() },
        Transform::from_xyz(0.0, -150.0, 0.0),
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

/// Linear transformation from domain `[-1.0, 1.0]` into range `[0.0, 1.0]`.
fn signed_weight_to_unsigned(x: f32) -> f32 {
    0.5 * (x + 1.0)
}

fn encode_weights(w: [u32; 4]) -> u32 {
    w[0] | (w[1] << 8) | (w[2] << 16) | (w[3] << 24)
}

fn sigmoid(x: f32, beta: f32) -> f32 {
    1.0 / (1.0 + (x / (1.0 - x)).powf(-beta))
}
