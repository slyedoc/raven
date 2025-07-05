
//! Triplanar Mapping and Material Blending (AKA Splatting) for Bevy Engine
//!
//! ![Screenshot](https://media.githubusercontent.com/media/bonsairobo/bevy_triplanar_splatting/main/examples/screen.png)
//!
//! # Scope
//!
//! This crate provides the
//! [`TriplanarMaterial`](triplanar_material::TriplanarMaterial), which is based
//! on `bevy_pbr`'s [`StandardMaterial`](bevy::pbr::StandardMaterial), but it
//! supports blending up to 4 materials using array textures and a new
//! [`ATTRIBUTE_MATERIAL_WEIGHTS`](triplanar_material::ATTRIBUTE_MATERIAL_WEIGHTS)
//! vertex attribute.
//!
//! # Implementation
//!
//! The triplanar material is implemented using the
//! [`AsBindGroup`](bevy::render::render_resource::AsBindGroup) derive macro and
//! the [`Material`](bevy::pbr::Material) trait. Most of the magic happens in
//! the shader code.
//!
//! Where possible, we reuse shader imports from [`bevy::pbr`](bevy::pbr) to
//! implement lighting effects. Sadly there are still some shader functions and
//! code blocks that are copy-pasted from Bevy; we are hoping to eliminate these
//! in the future to make this crate easier to maintain.
//!
//! The new shader code is mostly concerned with how array textures are sampled
//! and blended together. The techniques therein were sourced from the following
//! references:
//!
//! - Ben Golus, ["Normal Mapping for a Triplanar
//!   Shader"](https://bgolus.medium.com/normal-mapping-for-a-triplanar-shader-10bf39dca05a)
//! - Inigo Quilez, ["Biplanar
//!   Mapping"](https://iquilezles.org/articles/biplanar/)
//! - Colin Barré-Brisebois and Stephen Hill, ["Blending in
//!   Detail"](https://blog.selfshadow.com/publications/blending-in-detail/)
//!
//! # Road Map
//!
//! - [ ] per-layer uniform constants (e.g. "emissive", "metallic", etc.)
//! - [ ] support different texture per plane, using more layers
//! - [ ] blend materials using depth/height map
//!   - see ["Advanced Terrain Texture
//!     Splatting"](https://www.gamedeveloper.com/programming/advanced-terrain-texture-splatting)

use bevy::{
    asset::{embedded_asset, load_internal_asset, weak_handle}, pbr::{MaterialPipeline, MaterialPipelineKey, StandardMaterialFlags}, prelude::*, render::{
        mesh::{MeshVertexAttribute, MeshVertexBufferLayoutRef},
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupShaderType, Face, RenderPipelineDescriptor, ShaderRef,
            ShaderType, SpecializedMeshPipelineError, TextureFormat, VertexFormat,
        },
        texture::GpuImage,
    }
};

const TRIPLANAR_SHADER_HANDLE: Handle<Shader> = weak_handle!("d04b26a8-87a4-4305-8464-768251a0072e");
const BIPLANAR_SHADER_HANDLE: Handle<Shader> = weak_handle!("df2a2056-8dfc-4dee-b018-2571966558a8");

pub struct TriplanarMaterialPlugin;

impl Plugin for TriplanarMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<TriplanarMaterial>::default());
        // .add_plugins(MaterialPlugin::<ExtendedMaterial<StandardMaterial, Water>>::default())

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
        embedded_asset!(app, "shaders/triplanar_material_vert.wgsl");
        embedded_asset!(app, "shaders/triplanar_material_frag.wgsl");
    }
}



// TODO: per-layer metallic/emissive/etc
//
/// An extension of `bevy_pbr`'s `StandardMaterial` that supports triplanar
/// mapping and material splatting/blending.
///
/// In order to support splatting, we need to add the `"MaterialWeights"` vertex
/// attribute and give all textures dimension `"2d_array"`. Up to 4 layers are
/// supported by the shader. Material weights are encoded as 4 `u8` numbers that
/// get packed into a `u32`.
#[derive(AsBindGroup, Asset, Reflect, Debug, Clone)]
#[type_path = "raven_terrain::triplanar::TriplanarMaterial"]
#[bind_group_data(TriplanarMaterialKey)]
#[uniform(0, TriplanarMaterialUniform)]
#[reflect(Default, Debug)]
pub struct TriplanarMaterial {
    pub base_color: Color,

    #[texture(1, dimension = "2d_array")]
    #[sampler(2)]
    pub base_color_texture: Option<Handle<Image>>,

    pub emissive: Color,

    #[texture(3, dimension = "2d_array")]
    #[sampler(4)]
    pub emissive_texture: Option<Handle<Image>>,

    pub perceptual_roughness: f32,

    pub metallic: f32,

    #[texture(5, dimension = "2d_array")]
    #[sampler(6)]
    pub metallic_roughness_texture: Option<Handle<Image>>,

    #[doc(alias = "specular_intensity")]
    pub reflectance: f32,

    #[texture(9, dimension = "2d_array")]
    #[sampler(10)]
    pub normal_map_texture: Option<Handle<Image>>,

    pub flip_normal_map_y: bool,

    #[texture(7, dimension = "2d_array")]
    #[sampler(8)]
    pub occlusion_texture: Option<Handle<Image>>,

    pub double_sided: bool,

    #[reflect(ignore)]
    pub cull_mode: Option<Face>,

    pub unlit: bool,

    pub alpha_mode: AlphaMode,

    pub depth_bias: f32,

    pub uv_scale: f32,
}

impl Default for TriplanarMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::srgb(1.0, 1.0, 1.0),
            base_color_texture: None,
            emissive: Color::BLACK,
            emissive_texture: None,
            perceptual_roughness: 0.089,
            metallic: 0.01,
            metallic_roughness_texture: None,
            reflectance: 0.5,
            occlusion_texture: None,
            normal_map_texture: None,
            flip_normal_map_y: false,
            double_sided: false,
            cull_mode: Some(Face::Back),
            unlit: false,
            alpha_mode: AlphaMode::Opaque,
            depth_bias: 0.0,
            uv_scale: 1.0,
        }
    }
}

impl Material for TriplanarMaterial {
    fn vertex_shader() -> ShaderRef {
        "embedded://raven_terrain/shaders/triplanar_material_vert.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "embedded://raven_terrain/shaders/triplanar_material_frag.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            ATTRIBUTE_MATERIAL_WEIGHTS.at_shader_location(2),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        if key.bind_group_data.normal_map {
            // descriptor
            //     .fragment
            //     .as_mut()
            //     .unwrap()
            //     .shader_defs
            //     .push("STANDARDMATERIAL_NORMAL_MAP".into());
        }
        descriptor.primitive.cull_mode = key.bind_group_data.cull_mode;
        Ok(())
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    #[inline]
    fn depth_bias(&self) -> f32 {
        self.depth_bias
    }
}

pub const ATTRIBUTE_MATERIAL_WEIGHTS: MeshVertexAttribute = MeshVertexAttribute::new("MaterialWeights", 582540667, VertexFormat::Uint32);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TriplanarMaterialKey {
    normal_map: bool,
    cull_mode: Option<Face>,
}

impl From<&TriplanarMaterial> for TriplanarMaterialKey {
    fn from(material: &TriplanarMaterial) -> Self {
        TriplanarMaterialKey {
            normal_map: material.normal_map_texture.is_some(),
            cull_mode: material.cull_mode,
        }
    }
}

/// The GPU representation of the uniform data of a [`TriplanarMaterial`].

#[derive(Clone, Default, ShaderType)]
pub struct TriplanarMaterialUniform {
    #[allow(dead_code)]
    pub base_color: Vec4,
    pub emissive: Vec4,
    pub roughness: f32,
    pub metallic: f32,
    pub reflectance: f32,
    pub flags: u32,
    pub alpha_cutoff: f32,
    pub uv_scale: f32,
}

impl AsBindGroupShaderType<TriplanarMaterialUniform> for TriplanarMaterial {
    fn as_bind_group_shader_type(
        &self,
        images: &RenderAssets<GpuImage>,
    ) -> TriplanarMaterialUniform {
        let mut flags = StandardMaterialFlags::NONE;
        if self.base_color_texture.is_some() {
            flags |= StandardMaterialFlags::BASE_COLOR_TEXTURE;
        }
        if self.emissive_texture.is_some() {
            flags |= StandardMaterialFlags::EMISSIVE_TEXTURE;
        }
        if self.metallic_roughness_texture.is_some() {
            flags |= StandardMaterialFlags::METALLIC_ROUGHNESS_TEXTURE;
        }
        if self.occlusion_texture.is_some() {
            flags |= StandardMaterialFlags::OCCLUSION_TEXTURE;
        }
        if self.double_sided {
            flags |= StandardMaterialFlags::DOUBLE_SIDED;
        }
        if self.unlit {
            flags |= StandardMaterialFlags::UNLIT;
        }
        let has_normal_map = self.normal_map_texture.is_some();
        if has_normal_map {
            if let Some(texture) = images.get(self.normal_map_texture.as_ref().unwrap()) {
                match texture.texture_format {
                    // All 2-component unorm formats
                    TextureFormat::Rg8Unorm
                    | TextureFormat::Rg16Unorm
                    | TextureFormat::Bc5RgUnorm
                    | TextureFormat::EacRg11Unorm => {
                        flags |= StandardMaterialFlags::TWO_COMPONENT_NORMAL_MAP;
                    }
                    _ => {}
                }
            }
            if self.flip_normal_map_y {
                flags |= StandardMaterialFlags::FLIP_NORMAL_MAP_Y;
            }
        }
        // NOTE: 0.5 is from the glTF default - do we want this?
        let mut alpha_cutoff = 0.5;
        match self.alpha_mode {
            AlphaMode::Opaque => flags |= StandardMaterialFlags::ALPHA_MODE_OPAQUE,
            AlphaMode::Mask(c) => {
                alpha_cutoff = c;
                flags |= StandardMaterialFlags::ALPHA_MODE_MASK;
            }
            AlphaMode::Blend => flags |= StandardMaterialFlags::ALPHA_MODE_BLEND,
            AlphaMode::Premultiplied => flags |= StandardMaterialFlags::ALPHA_MODE_PREMULTIPLIED,
            AlphaMode::Add => flags |= StandardMaterialFlags::ALPHA_MODE_ADD,
            AlphaMode::Multiply => flags |= StandardMaterialFlags::ALPHA_MODE_MULTIPLY,
            AlphaMode::AlphaToCoverage => {
                flags |= StandardMaterialFlags::ALPHA_MODE_ALPHA_TO_COVERAGE
            }
        }

        TriplanarMaterialUniform {
            base_color: self.base_color.to_linear().to_vec4(),
            emissive: self.emissive.to_srgba().to_vec4(),
            roughness: self.perceptual_roughness,
            metallic: self.metallic,
            reflectance: self.reflectance,
            flags: flags.bits(),
            alpha_cutoff,
            uv_scale: self.uv_scale,
        }
    }
}
