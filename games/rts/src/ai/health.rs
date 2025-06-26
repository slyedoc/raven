use bevy::{
    asset::weak_handle, pbr::{MaterialPipeline, MaterialPipelineKey}, render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    }
};

use crate::prelude::*;

// pub const DEFAULT_BACKGROUND_COLOR: Color = Color::srgba(0., 0., 0., 0.75);
// pub const DEFAULT_BORDER_COLOR: Color = Color::srgba(0.02, 0.02, 0.02, 0.95);
// pub const DEFAULT_HIGH_COLOR: Color = Color::srgba(0., 1., 0., 0.95);
// pub const DEFAULT_MODERATE_COLOR: Color = Color::srgba(1., 1., 0., 0.95);
// pub const DEFAULT_LOW_COLOR: Color = Color::srgba(1., 0., 0., 0.95);

// pub const DEFAULT_WIDTH: f32 = 1.2;
// pub const DEFAULT_RELATIVE_HEIGHT: f32 = 0.1666;

pub(crate) const BAR_SHADER_HANDLE: Handle<Shader> = weak_handle!("f26e00b8-8957-4be8-995d-f27624aba19d");

#[derive(Component, Debug, Reflect, Clone, Copy, Default, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Percentage for Health {
    fn value(&self) -> f32 {
        self.current / self.max
    }
}

impl Health {
    pub fn new(health: f32) -> Self {
        Self {
            current: health,
            max: health,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn reset(&mut self) {
        self.current = self.max;
    }
}

/// System to handle damage events
#[derive(Event, Debug)]
pub struct Damage {
    pub amount: f32,
    #[allow(dead_code)]
    pub from: Entity,
}

/// Heros are set to despawn when their health reaches 0
pub fn on_damage(
    trigger: Trigger<Damage>,
    mut query: Query<(&mut Health, Has<Hero>), Without<Dead>>,
    mut commands: Commands,
) {
    let e = trigger.target();
    let event = trigger.event();
    let Ok((mut health, hero)) = query.get_mut(e) else {
        error!("Failed to get health for entity {:?}", e);
        return;
    };

    health.current = (health.current - event.amount).max(0.0);
    if health.is_dead() {
        match hero {
            true => {
                // mark player as dead
                commands
                    .entity(e)
                    .insert(Dead(Timer::from_seconds(5.0, TimerMode::Once)));
            }
            false => {
                // despawn entity
                commands.entity(e).despawn();
            }
        }
    }
}

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect)]
#[bind_group_data(BarMaterialKey)]
pub(crate) struct BarMaterial {
    #[uniform(0)]
    pub value_and_dimensions: Vec4,
    // (value, width, height, border_width) vec4 to be 16byte aligned
    #[uniform(1)]
    pub background_color: LinearRgba,
    #[uniform(2)]
    pub high_color: LinearRgba,
    #[uniform(3)]
    pub moderate_color: LinearRgba,
    #[uniform(4)]
    pub low_color: LinearRgba,
    #[uniform(5)]
    pub offset: Vec4,
    #[uniform(6)]
    pub border_color: LinearRgba,
    pub vertical: bool,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub(crate) struct BarMaterialKey {
    vertical: bool,
    border: bool,
}

impl From<&BarMaterial> for BarMaterialKey {
    fn from(material: &BarMaterial) -> Self {
        Self {
            vertical: material.vertical,
            border: material.value_and_dimensions.w > 0.,
        }
    }
}

impl Material for BarMaterial {
    fn vertex_shader() -> ShaderRef {
        BAR_SHADER_HANDLE.into()
    }

    fn fragment_shader() -> ShaderRef {
        BAR_SHADER_HANDLE.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
        ])?;

        let fragment = descriptor.fragment.as_mut().unwrap();
        if key.bind_group_data.vertical {
            fragment.shader_defs.push("IS_VERTICAL".into());
        }

        if key.bind_group_data.border {
            fragment.shader_defs.push("HAS_BORDER".into());
        }

        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
