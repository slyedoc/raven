use std::fmt;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_asset_loader::prelude::*;

// Uses to test different material features at runtime, based on parallax mapping example in bevy
// using [parallax](https://github.com/bevyengine/bevy/commit/8df014fbaffe9b7c21e4c74c8b10341d55a60364), see 
#[derive(Resource)]
pub struct MaterialTester(pub Handle<StandardMaterial>);

#[derive(AssetCollection, Resource)]
pub struct CoastSandRock02 {
    #[asset(path = "coast_sand_rocks_02/coast_sand_rocks_02_diff_4k.png")]
    pub base_color: Handle<Image>,
    #[asset(path = "coast_sand_rocks_02/coast_sand_rocks_02_ao_4k.png")]
    pub occlusion: Handle<Image>,
    #[asset(path = "coast_sand_rocks_02/coast_sand_rocks_02_nor_gl_4k.png")]
    pub normal: Handle<Image>,
    #[asset(path = "coast_sand_rocks_02/coast_sand_rocks_02_arm_4k.png")]
    pub metal_rough: Handle<Image>,
    #[asset(path = "coast_sand_rocks_02/coast_sand_rocks_02_rough_4k.png")]
    pub rough: Handle<Image>,
    #[asset(path = "coast_sand_rocks_02/coast_sand_rocks_02_depth_4k.png")]
    pub depth_map: Handle<Image>,
}

pub struct MaterialTesterPlugin;

impl Plugin for MaterialTesterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_parallax_depth_scale,  //Digit1-2
                update_parallax_layers,//Digit3-4
                toggle_parallax.run_if(input_just_pressed(KeyCode::KeyF)),
                switch_method.run_if(input_just_pressed(KeyCode::KeyG)),
                toggle_occlusion_texture.run_if(input_just_pressed(KeyCode::KeyH)),
                toggle_metal.run_if(input_just_pressed(KeyCode::KeyJ)),
                toggle_diff_metals.run_if(input_just_pressed(KeyCode::KeyK)),                
                toggle_rough_images.run_if(input_just_pressed(KeyCode::KeyL)),
            ),
        );
    }
}

fn setup(mut commands: Commands) {
    let parallax_depth_scale = TargetDepth::default().0;
    let max_parallax_layer_count = ops::exp2(TargetLayers::default().0);
    let parallax_mapping_method = CurrentMethod::default();

    // example instructions
    commands
        .spawn((
            Text::default(),
            ParallaxText,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn(TextSpan(format!(
                "Parallax depth scale: {parallax_depth_scale:.5}\n"
            )));
            p.spawn(TextSpan(format!("Layers: {max_parallax_layer_count:.0}\n")));
            p.spawn(TextSpan(format!("{parallax_mapping_method}\n")));
            p.spawn(TextSpan::new("\n\n"));
            p.spawn(TextSpan::new("Controls:\n"));
            p.spawn(TextSpan::new("P - Toggle parallax depth map\n"));
            p.spawn(TextSpan::new(
                "1/2 - Decrease/Increase parallax depth scale\n",
            ));
            p.spawn(TextSpan::new("3/4 - Decrease/Increase layer count\n"));            
            p.spawn(TextSpan::new("F - Switch parallax method\n"));
            p.spawn(TextSpan::new("O - Toggle occlusion texture\n"));
            p.spawn(TextSpan::new("M - Toggle metal texture\n"));
            p.spawn(TextSpan::new("I - Toggle metal image\n"));            
            p.spawn(TextSpan::new("R - Toggle rough images\n"));
        });
}

#[allow(dead_code)]
pub fn toggle_occlusion_texture(
    handles: Res<CoastSandRock02>,
    toggle_thing: Res<MaterialTester>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mat = materials.get_mut(&toggle_thing.0).unwrap();

    if mat.occlusion_texture.is_some() {
        mat.occlusion_texture = None;
        info!("Material: Occlusion texture disabled");
    } else {
        mat.occlusion_texture = Some(handles.occlusion.clone());
        info!("Material: Occlusion texture enabled");
    };
}

#[allow(dead_code)]
pub fn toggle_metal(
    handles: Res<CoastSandRock02>,
    toggle_thing: Res<MaterialTester>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mat = materials.get_mut(&toggle_thing.0).unwrap();

    if mat.metallic_roughness_texture.is_some() {
        mat.metallic_roughness_texture = None;
        mat.metallic = 0.2; // set so metallic_roughness_texture is used
        mat.perceptual_roughness = 0.2; // set so metallic_roughness_texture is used
        info!("Material: Metallic and roughness texture disabled");
    } else {
        mat.metallic = 1.0; // set so metallic_roughness_texture is used
        mat.perceptual_roughness = 1.0; // set so metallic_
        mat.metallic_roughness_texture = Some(handles.metal_rough.clone());
        info!("Material: Metallic and roughness texture enabled");
    };
}

#[allow(dead_code)]
pub fn toggle_diff_metals(
    handles: Res<CoastSandRock02>,
    toggle_thing: Res<MaterialTester>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut local: Local<bool>,
) {
    let mat = materials.get_mut(&toggle_thing.0).unwrap();

    match *local {
        true => {
            mat.metallic_roughness_texture = Some(handles.metal_rough.clone());
            mat.metallic = 1.0;
            mat.perceptual_roughness = 1.0;
            info!("Material: metal_rough texture");
        }
        false => {
            mat.metallic = 1.0; // set so metallic_roughness_texture is used
            mat.perceptual_roughness = 1.0; // set so metallic_
            mat.metallic_roughness_texture = Some(handles.rough.clone());
            info!("Material: rough texture enabled");
        }
    }
    *local = !*local;
}

#[allow(dead_code)]
pub fn toggle_parallax(
    handles: Res<CoastSandRock02>,
    toggle_thing: Res<MaterialTester>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mat = materials.get_mut(&toggle_thing.0).unwrap();

    if mat.depth_map.is_some() {
        mat.depth_map = None;
        info!("Material: Parallax depth map disabled");
    } else {
        mat.depth_map = Some(handles.depth_map.clone());
        info!("Material: Parallax depth map enabled");
    }
}

#[allow(dead_code)]
pub fn toggle_rough_images(
    handles: Res<CoastSandRock02>,
    toggle_thing: Res<MaterialTester>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut local: Local<bool>,
) {
    let mat = materials.get_mut(&toggle_thing.0).unwrap();

    match *local {
        true => {
            mat.metallic_roughness_texture = Some(handles.metal_rough.clone());
            mat.metallic = 1.0;
            mat.perceptual_roughness = 1.0;
            info!("Material: metal_rough texture");
        }
        false => {
            mat.metallic = 1.0; // set so metallic_roughness_texture is used
            mat.perceptual_roughness = 1.0; // set so metallic_
            mat.metallic_roughness_texture = Some(handles.rough.clone());
            info!("Material: rough texture enabled");
        }
    }
    *local = !*local;
}

const DEPTH_CHANGE_RATE: f32 = 0.1;
const DEPTH_UPDATE_STEP: f32 = 0.03;
const MAX_DEPTH: f32 = 0.3;

pub struct TargetDepth(pub(crate) f32);
impl Default for TargetDepth {
    fn default() -> Self {
        TargetDepth(0.09)
    }
}
pub struct TargetLayers(pub(crate) f32);
impl Default for TargetLayers {
    fn default() -> Self {
        TargetLayers(5.0)
    }
}
pub struct CurrentMethod(pub(crate) ParallaxMappingMethod);
impl Default for CurrentMethod {
    fn default() -> Self {
        CurrentMethod(ParallaxMappingMethod::Relief { max_steps: 4 })
    }
}
impl fmt::Display for CurrentMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ParallaxMappingMethod::Occlusion => write!(f, "Parallax Occlusion Mapping"),
            ParallaxMappingMethod::Relief { max_steps } => {
                write!(f, "Relief Mapping with {max_steps} steps")
            }
        }
    }
}
impl CurrentMethod {
    fn next_method(&mut self) {
        use ParallaxMappingMethod::*;
        self.0 = match self.0 {
            Occlusion => Relief { max_steps: 2 },
            Relief { max_steps } if max_steps < 3 => Relief { max_steps: 4 },
            Relief { max_steps } if max_steps < 5 => Relief { max_steps: 8 },
            Relief { .. } => Occlusion,
        }
    }
}

pub fn update_parallax_depth_scale(
    input: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut target_depth: Local<TargetDepth>,
    mut depth_update: Local<bool>,
    mut writer: TextUiWriter,
    text: Single<Entity, (With<Text>, With<ParallaxText>)>,
) {
    if input.just_pressed(KeyCode::Digit1) {
        target_depth.0 -= DEPTH_UPDATE_STEP;
        target_depth.0 = target_depth.0.max(0.0);
        *depth_update = true;
    }
    if input.just_pressed(KeyCode::Digit2) {
        target_depth.0 += DEPTH_UPDATE_STEP;
        target_depth.0 = target_depth.0.min(MAX_DEPTH);
        *depth_update = true;
    }
    if *depth_update {
        for (_, mat) in materials.iter_mut() {
            let current_depth = mat.parallax_depth_scale;
            let new_depth = current_depth.lerp(target_depth.0, DEPTH_CHANGE_RATE);
            mat.parallax_depth_scale = new_depth;
            *writer.text(*text, 1) = format!("Parallax depth scale: {new_depth:.5}\n");
            if (new_depth - current_depth).abs() <= 0.000000001 {
                *depth_update = false;
            }
        }
    }
}

pub fn switch_method(
    mut materials: ResMut<Assets<StandardMaterial>>,
    text: Single<Entity, (With<Text>, With<ParallaxText>)>,
    mut writer: TextUiWriter,
    mut current: Local<CurrentMethod>,
) {
    current.next_method();
    info!("Switched parallax mapping method to: {:?}", current.0);

    let text_entity = *text;
    *writer.text(text_entity, 3) = format!("Method: {}\n", *current);

    for (_, mat) in materials.iter_mut() {
        mat.parallax_mapping_method = current.0;
    }
}

#[derive(Component)]
pub struct ParallaxText;

pub fn update_parallax_layers(
    input: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut target_layers: Local<TargetLayers>,
    text: Single<Entity, (With<Text>, With<ParallaxText>)>,
    mut writer: TextUiWriter,
) {
    if input.just_pressed(KeyCode::Digit3) {
        target_layers.0 -= 1.0;
        target_layers.0 = target_layers.0.max(0.0);
    } else if input.just_pressed(KeyCode::Digit4) {
        target_layers.0 += 1.0;
    } else {
        return;
    }
    let layer_count = ops::exp2(target_layers.0);
    let text_entity = *text;
    *writer.text(text_entity, 2) = format!("Layers: {layer_count:.0}\n");

    for (_, mat) in materials.iter_mut() {
        mat.max_parallax_layer_count = layer_count;
    }
}
