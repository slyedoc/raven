pub mod config;
pub use config::*;
use material::BarMaterial;
use mesh::MeshHandles;

mod material;
mod mesh;

use std::marker::PhantomData;

use bevy::asset::{load_internal_asset, weak_handle};
use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;

pub const DEFAULT_BACKGROUND_COLOR: Color = Color::srgba(0., 0., 0., 0.75);
pub const DEFAULT_BORDER_COLOR: Color = Color::srgba(0.02, 0.02, 0.02, 0.95);
pub const DEFAULT_HIGH_COLOR: Color = Color::srgba(0., 1., 0., 0.95);
pub const DEFAULT_MODERATE_COLOR: Color = Color::srgba(1., 1., 0., 0.95);
pub const DEFAULT_LOW_COLOR: Color = Color::srgba(1., 0., 0., 0.95);

pub const DEFAULT_WIDTH: f32 = 1.2;
pub const DEFAULT_RELATIVE_HEIGHT: f32 = 0.1666;

pub(crate) const BAR_SHADER_HANDLE: Handle<Shader> = weak_handle!("8c3a684e-3e75-47e7-b5b5-99fbc323c219");

pub struct BarPlugin<T: Percentage + Component + TypePath> {
    phantom: PhantomData<T>,
}

impl<T: Percentage + Component + TypePath> Default for BarPlugin<T> {
    fn default() -> Self {
        Self {
            phantom: Default::default(),
        }
    }
}

impl<T: Percentage + Component + TypePath> Plugin for BarPlugin<T> {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<MaterialPlugin<BarMaterial>>() {
            app.add_plugins(MaterialPlugin::<BarMaterial>::default());

            load_internal_asset!(app, BAR_SHADER_HANDLE, "bar.wgsl", Shader::from_wgsl);
        }

        app.init_resource::<MeshHandles>()
            .init_resource::<ColorScheme<T>>()
            .register_type::<BarSettings<T>>()
            .add_systems(PostUpdate, reset_rotation)
            .add_systems(
                Update,
                (spawn::<T>, remove::<T>, update::<T>, update_settings::<T>),
            );
    }
}

#[derive(Component, Reflect)]
struct WithBar<T: Percentage + Component>(Entity, #[reflect(ignore)] PhantomData<T>);

impl<T: Percentage + Component> WithBar<T> {
    fn get(&self) -> Entity {
        self.0
    }
}

#[allow(clippy::type_complexity)]
fn spawn<T: Percentage + Component + TypePath>(
    mut commands: Commands,
    mut materials: ResMut<Assets<BarMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_handles: ResMut<MeshHandles>,
    color_scheme: Res<ColorScheme<T>>,
    query: Query<(Entity, &T, &BarSettings<T>), Added<T>>,
) {
    query.iter().for_each(|(entity, percentage, settings)| {
        let width = settings.normalized_width();
        let height = settings.normalized_height();

        let mesh = mesh_handles.get(width, height).cloned().unwrap_or_else(|| {
            mesh_handles.insert(
                width,
                height,
                meshes.add(Mesh::from(Rectangle::new(width, height))),
            )
        });

        let (high, moderate, low) = match color_scheme.foreground_color {
            ForegroundColor::Static(color) => (color, color, color),
            ForegroundColor::TriSpectrum {
                high,
                moderate,
                low,
            } => (high, moderate, low),
        };

        let material = materials.add(BarMaterial {
            value_and_dimensions: (percentage.value(), width, height, settings.border.width).into(),
            background_color: color_scheme.background_color.into(),
            high_color: high.into(),
            moderate_color: moderate.into(),
            low_color: low.into(),
            vertical: settings.orientation == BarOrientation::Vertical,
            offset: settings.normalized_offset().extend(0.),
            border_color: settings.border.color.into(),
        });

        let health_bar = commands
            .spawn((
                Name::new(format!("{}Bar", T::type_path())),
                Mesh3d(mesh),
                MeshMaterial3d(material),
                NotShadowCaster,
                NotShadowReceiver,
            ))
            .id();

        commands
            .entity(entity)
            .insert(WithBar(health_bar, PhantomData::<T>))
            .add_child(health_bar);
    });
}

fn update<T: Percentage + Component + TypePath>(
    mut materials: ResMut<Assets<BarMaterial>>,
    parent_query: Query<(&WithBar<T>, &T), Changed<T>>,
    bar_query: Query<&MeshMaterial3d<BarMaterial>>,
) {
    parent_query.iter().for_each(|(bar, percentage)| {
        let Ok(material_handle) = bar_query.get(bar.get()) else {
            return;
        };
        let material = materials.get_mut(material_handle).unwrap();
        material.value_and_dimensions.x = percentage.value();
    });
}

#[allow(clippy::type_complexity)]
fn update_settings<T: Percentage + Component + TypePath>(
    mut commands: Commands,
    mut materials: ResMut<Assets<BarMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_handles: ResMut<MeshHandles>,
    parent_query: Query<(&WithBar<T>, &BarSettings<T>), Changed<BarSettings<T>>>,
    bar_query: Query<(Entity, &MeshMaterial3d<BarMaterial>, &Mesh3d)>,
) {
    parent_query.iter().for_each(|(bar, settings)| {
        let Ok((entity, material_handle, mesh_handle)) = bar_query.get(bar.get()) else {
            return;
        };

        let material = materials.get_mut(material_handle).unwrap();
        let offset = settings.normalized_offset().extend(0.);
        let width = settings.normalized_width();
        let height = settings.normalized_height();

        let mesh_for_settings_dimensions = mesh_handles.get(width, height).cloned();
        let mesh_changed = mesh_for_settings_dimensions
            .clone()
            .map_or(true, |handle| handle != mesh_handle.0);

        if mesh_changed {
            let new_mesh = mesh_for_settings_dimensions.unwrap_or(mesh_handles.insert(
                width,
                height,
                meshes.add(Mesh::from(Rectangle::new(width, height))),
            ));
            commands.entity(entity).insert(Mesh3d(new_mesh));
            material.value_and_dimensions.y = width;
            material.value_and_dimensions.z = height;
        }

        material.offset = offset;
        material.border_color = settings.border.color.into();
        material.value_and_dimensions.w = settings.border.width;
        material.vertical = settings.orientation == BarOrientation::Vertical;
    });
}

fn remove<T: Percentage + Component>(
    mut commands: Commands,
    mut removals: RemovedComponents<T>,
    parent_query: Query<&WithBar<T>>,
) {
    removals.read().for_each(|entity| {
        let Ok(&WithBar(bar_entity, _)) = parent_query.get(entity) else {
            return;
        };

        if commands.get_entity(bar_entity).is_ok() {
            return;
        }

        commands.entity(bar_entity).despawn()
    });
}

fn reset_rotation(
    mut bar_query: Query<(&ChildOf, &mut Transform), With<MeshMaterial3d<BarMaterial>>>,
    q_transform: Query<&Transform, Without<MeshMaterial3d<BarMaterial>>>,
) {
    for (parent, mut transform) in bar_query.iter_mut() {
        if let Ok(parent_transform) = q_transform.get(parent.parent()) {
            transform.rotation = parent_transform.rotation.inverse();
        }
    }
}
