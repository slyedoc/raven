//! Demonstrates bindless `ExtendedMaterial`.


use bevy::{
    color::palettes::{
        css,
        tailwind,
    },
    pbr::{ExtendedMaterial, MeshMaterial3d},
    prelude::*,    
};
use raven_editor::prelude::*;
use raven_splat::prelude::*;
use raven_util::prelude::*;

use bevy_asset_loader::prelude::*;

/// The entry point.
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TriplanarMaterialPlugin,
            CameraFreePlugin,
            EditorPlugin::default(),
        ))
        .init_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading)
                .continue_to_state(AppState::Main)
                .load_collection::<MaterialHandles>(),
        )
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::Main), (
            setup_with_assets, // new shader
            setup_with_assets_old, // old shader

        ))
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum AppState {
    #[default]
    AssetLoading,
    Main,
}

#[derive(AssetCollection, Resource)]
pub struct MaterialHandles {
    #[asset(path = "array_textures/base_color.ktx2")]
    #[asset(image(sampler(filter = linear, wrap = repeat)))]
    pub base_color: Handle<Image>,
    #[asset(path = "array_textures/occlusion.ktx2")]
    #[asset(image(sampler(filter = linear, wrap = repeat)))]
    pub occlusion: Handle<Image>,
    #[asset(path = "array_textures/normal.ktx2")]
    #[asset(image(sampler(filter = linear, wrap = repeat)))]
    pub normal_map: Handle<Image>,
    #[asset(path = "array_textures/metal_rough.ktx2")]
    #[asset(image(sampler(filter = linear, wrap = repeat)))]
    pub metal_rough: Handle<Image>,    
    #[asset(path = "array_textures/depth_map.ktx2")]
    #[asset(image(sampler(filter = linear, wrap = repeat)))]
    pub depth_map: Handle<Image>,
}

/// Creates the scene.
///! Here's is a link to the [Header](crate::module_a#header) section.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a light.
    commands.spawn((
        Name::new("Main Light"),
        DirectionalLight::default(),
        Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // ground
    commands.spawn((
        Name::new("Ground"),
        Mesh3d(
            meshes.add(
                Plane3d::new(Vec3::Y, Vec2::splat(5.0))
                    .mesh()
                    .subdivisions(4)
                    .build(),
            ),
        ),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: tailwind::GRAY_600.into(),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Create a camera.
    commands.spawn((
        Name::new("Main Camera"),
        Camera3d::default(),
        CameraFree,
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_with_assets(
    mut commands: Commands,
    handles: Res<MaterialHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut splat_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, SplatExtension>>>,
) {

    let mut plane_mesh = Plane3d::new(Vec3::Y, Vec2::splat(5.0))
        .mesh()
        .subdivisions(4)
        .build();
    let plane_weights = plane_mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .unwrap()
        .as_float3()
        .unwrap()
        .iter()
        .map(|p| {
            // use the x coordinate to determine the weight
            let p = Vec3::from(*p);
            let (w0, w1) = match p.x > 0.0 {
                true => (255, 0),
                false => (0, 255),
            };
            encode_weights([w0, 0, w1, 0])
        })
        .collect::<Vec<_>>();
    plane_mesh.insert_attribute(ATTRIBUTE_MATERIAL_WEIGHTS, plane_weights);
    
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0))),
        MeshMaterial3d(splat_materials.add(ExtendedMaterial {
            base: StandardMaterial {                                                
                metallic: 1.0,             // Set so metallic_roughness_texture is used
                perceptual_roughness: 1.0, // Set so metallic_roughness_texture is used
                ..default()
            },
            extension: SplatExtension {
                modulate_color: css::RED.into(),

                base_color_texture: Some(handles.base_color.clone()),
                metallic_roughness_texture: Some(handles.metal_rough.clone()),
                normal_map_texture: Some(handles.normal_map.clone()),
                occlusion_texture: Some(handles.occlusion.clone()),
            },
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));
}


fn setup_with_assets_old(
    mut commands: Commands,    
    handles: ResMut<MaterialHandles>,
    mut planer_materials: ResMut<Assets<TriplanarMaterial>>,

    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut sphere_mesh =Sphere::new(1.0).mesh().ico(6).unwrap();

    let sphere_weights: Vec<u32> = sphere_mesh
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
    sphere_mesh.insert_attribute(ATTRIBUTE_MATERIAL_WEIGHTS, sphere_weights);
        
    commands.spawn((
        Mesh3d(meshes.add(sphere_mesh)),        
        MeshMaterial3d(planer_materials.add(TriplanarMaterial {
            base_color_texture: Some(handles.base_color.clone()),
            emissive_texture: None,            
            metallic: 1.0,             // Set so metallic_roughness_texture is used
            perceptual_roughness: 1.0, // Set so metallic_roughness_texture is used
            metallic_roughness_texture: Some(handles.metal_rough.clone()),

            normal_map_texture: Some(handles.normal_map.clone()),
            occlusion_texture: Some(handles.occlusion.clone()),
            uv_scale: 1.0,
            ..default()
        })),
        Transform::from_xyz(-10.0, 1.0, 0.0),
    ));

    let mut plane_mesh = Plane3d::new(Vec3::Y, Vec2::splat(5.0)).mesh().subdivisions(4).build();
    let plane_weights = plane_mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .unwrap()
        .as_float3()
        .unwrap()
        .iter()
        .map(|p| {
            // use the x coordinate to determine the weight
            let p = Vec3::from(*p);
            let (w0, w1) = match p.x > 0.0 {

                true => (255, 0),
                false => (0, 255),
            };                 
            encode_weights([w0, 0, w1, 0])
        })
        .collect::<Vec<_>>();
        plane_mesh.insert_attribute(ATTRIBUTE_MATERIAL_WEIGHTS, plane_weights);

    commands.spawn((
        Mesh3d(meshes.add(plane_mesh)),
        MeshMaterial3d(planer_materials.add(TriplanarMaterial {
            base_color_texture: Some(handles.base_color.clone()),
            emissive_texture: None,
            metallic: 1.0,             // Set so metallic_roughness_texture is used
            perceptual_roughness: 1.0, // Set so metallic_roughness_texture is used
            metallic_roughness_texture: Some(handles.metal_rough.clone()),

            normal_map_texture: Some(handles.normal_map.clone()),
            occlusion_texture: Some(handles.occlusion.clone()),

            uv_scale: 1.0,
            ..default()
        })),
        Transform::from_xyz(-20.0, 0.0, 0.0)
    ));
            
}

fn encode_weights(w: [u32; 4]) -> u32 {
    w[0] | (w[1] << 8) | (w[2] << 16) | (w[3] << 24)
}

/// Linear transformation from domain `[-1.0, 1.0]` into range `[0.0, 1.0]`.
fn signed_weight_to_unsigned(x: f32) -> f32 {
    0.5 * (x + 1.0)
}

fn sigmoid(x: f32, beta: f32) -> f32 {
    1.0 / (1.0 + (x / (1.0 - x)).powf(-beta))
}
