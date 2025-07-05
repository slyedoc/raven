use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

/// A simple material that displays UV coordinates as colors
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FooMaterial {
    #[uniform(100)]
    pub base_color: f32,
}

impl MaterialExtension for FooMaterial {
    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/foo_material.wgsl".into()
    }
}

pub type FooExtendedMaterial = ExtendedMaterial<StandardMaterial, FooMaterial>;

pub struct FooMaterialPlugin;

impl Plugin for FooMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<FooExtendedMaterial>::default());
    }
}