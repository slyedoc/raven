use bevy::render::camera::ScalingMode;

use crate::prelude::*;

const MINIMAP_LAYERS: RenderLayers = RenderLayers::layer(1);

// The projection used for the camera in 2D
// const MINIMAP_PROJECTION_2D: Projection = Projection::Orthographic(OrthographicProjection {
//     near: -1.0,
//     far: 10.0,
//     scale: 1.0,
//     viewport_origin: Vec2::new(0.5, 0.5),
//     scaling_mode: ScalingMode::AutoMax {
//         max_width: 8.0,
//         max_height: 20.0,
//     },
//     area: Rect {
//         min: Vec2::NEG_ONE,
//         max: Vec2::ONE,
//     },
// });

// The transform of the camera in 2D
// Transform::from_translation(Vec3::new(0.0, 60.0, 0.0)).looking_at(Vec3::ZERO, -Vec3::Z),
// const MINIMAP_TRANSFORM_2D: Transform = Transform {
//     translation: Vec3::ZERO,
//     rotation: Quat::IDENTITY,
//     scale: Vec3::ONE,
// };

pub fn plugin(app: &mut App) {
    app.add_observer(on_add_minimap_icon)
        .add_systems(Startup, setup_minimap);
}

/// global resource for the minimap
#[derive(Resource)]
pub struct Minimap {
    pub image: Handle<Image>,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub assets: Vec<(MinimapIcon, (Handle<Mesh>, Handle<StandardMaterial>))>,
}

impl Default for Minimap {
    fn default() -> Self {
        Self {
            image: Default::default(),
            width: 0.0,
            height: 0.0,
            scale: 1.0,
            assets: Vec::new(),
        }
    }
}

impl Minimap {
    /// Get the mesh and material for a minimap icon
    fn get(&self, minimap: &MinimapIcon) -> Option<(Handle<Mesh>, Handle<StandardMaterial>)> {
        self.assets
            .iter()
            .find(|(icon, _)| icon.radius == minimap.radius && icon.color == minimap.color)
            .map(|(_, (mesh, mat))| (mesh.clone(), mat.clone()))
    }
}

/// Can be added to be rendered on the minimap
#[derive(Component, Debug, Clone, Copy)]
pub struct MinimapIcon {
    pub radius: f32,
    pub color: Color,
}

impl MinimapIcon {
    pub fn new(radius: f32, color: Color) -> Self {
        Self { radius, color }
    }
}

pub fn setup_minimap(mut commands: Commands, mut images: ResMut<Assets<Image>>, map_size: Res<MapSize>) {
    let max_size = 256;
    let aspect_ratio = map_size.0.x / map_size.0.y;

    let (width, height) = if aspect_ratio > 1.0 {
        // Wider than tall
        (max_size as f32, max_size as f32 / aspect_ratio)
    } else {
        // Taller than wide or square
        (max_size as f32 * aspect_ratio, max_size as f32)
    };
    let size = Extent3d {
        width: width.round() as u32,
        height: height.round() as u32,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);
    let image_handle = images.add(image);

    commands.insert_resource(Minimap {
        image: image_handle.clone(),
        width,
        height,
        ..default()
    });
    let half_map_size = map_size.half() / 10.0;
    commands.spawn((
        Name::new("Minimap Camera"),
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            near: 0.0,
            scaling_mode: ScalingMode::Fixed {
                width: width / 2.,
                height: height / 2.,
            },
            area: Rect::new(
                -half_map_size.x,
                -half_map_size.y,
                half_map_size.x,
                half_map_size.y,
            ),
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(Vec3::new(0.0, 500.0, 0.0)) // Elevate the camera
            .looking_at(Vec3::ZERO, -Vec3::Z),
        Camera {
            order: -1,
            target: image_handle.clone().into(),
            clear_color: Color::BLACK.into(),
            ..default()
        },
        MINIMAP_LAYERS,
        StateScoped(AppState::InGame),
    ));
}

fn on_add_minimap_icon(
    trigger: Trigger<OnAdd, MinimapIcon>,
    query: Query<&MinimapIcon>,
    mut commands: Commands,
    mut ass: ResMut<Minimap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let e = trigger.target();
    let minimap = query.get(e).unwrap();

    commands.entity(e).with_children(|b| {
        let (mesh, mat) = match ass.get(minimap) {
            Some(a) => a,
            None => {
                // create new handles
                let mesh = meshes.add(Circle::new(minimap.radius * ass.scale).mesh());
                let mat = materials.add(StandardMaterial {
                    base_color: minimap.color,
                    unlit: true,
                    ..default()
                });
                // add to cache
                ass.assets.push((*minimap, (mesh.clone(), mat.clone())));
                (mesh, mat)
            }
        };

        b.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(mat),
            MINIMAP_LAYERS,
            Transform {
                translation: Vec3::new(0., 0.0, 0.),
                rotation: Quat::from_rotation_arc(Vec3::Z, Vec3::Y),
                ..default()
            },
        ));
    });
}
