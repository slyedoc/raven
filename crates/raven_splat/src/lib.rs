#![allow(warnings)]

// Objectives for this shader to:
// 1. A material splatting/blending.

pub mod triplanar_material;
use crate::triplanar_material::TriplanarMaterial;
use bevy::asset::{embedded_asset, load_internal_asset, weak_handle};
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};

pub mod prelude {
    pub use crate::triplanar_material::{TriplanarMaterial, ATTRIBUTE_MATERIAL_WEIGHTS};
    pub use crate::*;    
}

const TRIPLANAR_SHADER_HANDLE: Handle<Shader> = weak_handle!("b53b7772-c357-41b1-a237-2932bc35b048");
const BIPLANAR_SHADER_HANDLE: Handle<Shader> = weak_handle!("15413827-466f-4117-9eb8-18b07607fe4c");
const PRB_FRAGMENT_SPLAT_SHADER_HANDLE: Handle<Shader> = weak_handle!("39df399d-71ed-421c-bd93-630ac739044a");

pub struct TriplanarMaterialPlugin;

impl Plugin for TriplanarMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<TriplanarMaterial>::default())
            .add_plugins(MaterialPlugin::<ExtendedMaterial<StandardMaterial, SplatExtension>,>::default())
            .register_type::<TriplanarMaterial>()
            .register_type::<SplatExtension>();

            
        load_internal_asset!(
            app,
            TRIPLANAR_SHADER_HANDLE,
            "shaders/triplanar.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            BIPLANAR_SHADER_HANDLE,
            "shaders/biplanar.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            PRB_FRAGMENT_SPLAT_SHADER_HANDLE,
            "shaders/pbr_fragment.wesl",
            Shader::from_wgsl
        );

        // embedded_asset!(app, "shaders/triplanar_material_vert.wgsl");
        // embedded_asset!(app, "shaders/triplanar_material_frag.wgsl");
    }
}

// See https://github.com/bevyengine/bevy/blob/main/examples/shader/extended_material_bindless.rs
// for binding information.
#[derive(Asset, Clone, Reflect, AsBindGroup)]
#[data(50, SplatUniform, binding_array(101))]
#[bindless(index_table(range(50..59), binding(100)))]
pub struct SplatExtension {
    /// The color we're going to multiply the base color with.
    pub modulate_color: Color,
    /// The image we're going to multiply the base color with.
    //#[texture(51)]
    //#[sampler(52)]
    //pub modulate_texture: Option<Handle<Image>>,

    #[texture(51, dimension = "2d_array")]
    #[sampler(52)]
    pub base_color_texture: Option<Handle<Image>>,

    #[texture(53, dimension = "2d_array")]
    #[sampler(54)]
    pub metallic_roughness_texture: Option<Handle<Image>>,

    #[texture(55, dimension = "2d_array")]
    #[sampler(56)]
    pub normal_map_texture: Option<Handle<Image>>,

    #[texture(57, dimension = "2d_array")]
    #[sampler(58)]
    pub occlusion_texture: Option<Handle<Image>>,
}

/// The GPU-side data structure specifying plain old data for the material
/// extension.
#[derive(Clone, Default, ShaderType)]
struct SplatUniform {
    /// The GPU representation of the color we're going to multiply the base
    /// color with.    
    modulate_color: Vec4,
}

impl MaterialExtension for SplatExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/splat_frag.wgsl".into()
    }

    // fn vertex_shader() -> ShaderRef {
    //     "shaders/splat_vert.wgsl".into()
    // }
}

impl<'a> From<&'a SplatExtension> for SplatUniform {
    fn from(material_extension: &'a SplatExtension) -> Self {
        // Convert the CPU `ExampleBindlessExtension` structure to its GPU
        // format.
        SplatUniform {
            modulate_color: LinearRgba::from(material_extension.modulate_color).to_vec4(),
        }
    }
}
