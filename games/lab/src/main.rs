use bevy::{
    color::palettes::tailwind, core_pipeline::Skybox,
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use bevy_simple_subsecond_system::prelude::*;
use raven_editor::prelude::*;
use raven_util::prelude::*;
mod foo_material;
use foo_material::*;
mod material_tester;
use material_tester::*;
use bevy_mod_mipmap_generator::*;

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
            EditorPlugin::default(),
            FooMaterialPlugin,
              MipmapGeneratorPlugin,
            MaterialTesterPlugin, // testing different material features
        ))
        .add_systems(Update, generate_mipmaps::<StandardMaterial>)
        .init_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading)
                .continue_to_state(AppState::AssetProcessing)
                .load_collection::<CoastSandRock02>(),
        )
        .add_systems(OnEnter(AppState::AssetProcessing), fix_assets)
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::Main), setup_with_assets)                
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum AppState {
    #[default]
    AssetLoading,
    AssetProcessing,
    Main,
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut foo_materials: ResMut<Assets<FooExtendedMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraFree,
        Skybox {
            brightness: 5000.0,
            image: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            ..default()
        },
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 900.0,
            ..default()
        },
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 1_500.0,
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

fn fix_assets(
    _handles: Res<CoastSandRock02>,
    mut _images: ResMut<Assets<Image>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    // for _handle in [
    //     &handles.base_color,
    //     // &handles.occlusion,
    //     // &handles.normal_map,
    //     // &handles.metal_rough,
    // ] {
    //     // let image = images.get_mut(handle).unwrap();

    //     // Create a new array texture asset from the loaded texture.
    //     // let array_layers = 4;
    //     // image.reinterpret_stacked_2d_as_array(array_layers);
    // }

    app_state.set(AppState::Main);
}

fn setup_with_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    handles: Res<CoastSandRock02>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh = Plane3d::default().mesh().size(10.0, 10.0).build();

    match mesh.generate_tangents() {
        Ok(_) => (),
        Err(x) => eprintln!("Error generating tangents: {:?}", x),
    };    

    let mat = materials.add(StandardMaterial {
        //perceptual_roughness: 0.4,
        base_color_texture: Some(handles.base_color.clone()),

        // The blue channel contains metallic values,
        // and the green channel contains the roughness values.
        // Other channels are unused.
        metallic: 1.0,             // set so metallic_roughness_texture is used
        perceptual_roughness: 1.0, // set so metallic_roughness_texture is used
        metallic_roughness_texture: Some(handles.metal_rough.clone()),

        // requires the following
        // - A normal map texture
        // - Vertex UVs
        // - Vertex tangents (Most likely the one missing)
        // - Vertex normals
        normal_map_texture: Some(handles.normal.clone()),
        occlusion_texture: Some(handles.occlusion.clone()),

        // The depth map is a grayscale texture where black is the highest level and
        // white the lowest.
        depth_map: Some(handles.depth_map.clone()),
        parallax_depth_scale: 0.03,

        parallax_mapping_method: ParallaxMappingMethod::Relief { max_steps: 4 },
        max_parallax_layer_count: ops::exp2(4.0), // 16.0
        // parallax_depth_scale: 0.09,
        // parallax_mapping_method: ParallaxMappingMethod::Relief { max_steps: 4 },
        // max_parallax_layer_count: ops::exp2(5.0f32),
        ..default()
    });

    commands.insert_resource(MaterialTester(mat.clone()));

    commands.spawn((
        Name::new("Cost Sand Rock 02"),
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(mat),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // The normal map. Note that to generate it in the GIMP image editor, you should
    // open the depth map, and do Filters → Generic → Normal Map
    // You should enable the "flip X" checkbox.
    // let normal_handle = asset_server.load_with_settings(
    //     "textures/parallax_example/cube_normal.png",
    //     // The normal map texture is in linear color space. Lighting won't look correct
    //     // if `is_srgb` is `true`, which is the default.
    //     |settings: &mut ImageLoaderSettings| settings.is_srgb = false,
    // );

    //   let parallax_depth_scale = TargetDepth::default().0;
    // let max_parallax_layer_count = ops::exp2(TargetLayers::default().0);
    // let parallax_mapping_method = CurrentMethod::default();
    // let parallax_material = materials.add(StandardMaterial {
    //     perceptual_roughness: 0.4,
    //     base_color_texture: Some(asset_server.load("textures/parallax_example/cube_color.png")),
    //     normal_map_texture: Some(normal_handle),
    //     // The depth map is a grayscale texture where black is the highest level and
    //     // white the lowest.
    //     depth_map: Some(asset_server.load("textures/parallax_example/cube_depth.png")),
    //     parallax_depth_scale,
    //     parallax_mapping_method: parallax_mapping_method.0,
    //     max_parallax_layer_count,
    //     ..default()
    // });
}
