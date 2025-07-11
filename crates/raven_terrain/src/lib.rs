#![allow(unused_imports)]
mod debug;
mod quadtree;
//mod water;


use avian3d::prelude::*;
use bevy::{
    asset::RenderAssetUsages,
    color::palettes::tailwind,
    math::bounding::{Aabb3d, BoundingVolume},
    platform::{collections::HashMap, hash::FixedHasher},
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use bevy_inspector_egui::{inspector_options::std_options::NumberDisplay, prelude::*};
use bevy_rand::prelude::*;
use raven_splat::prelude::*;
use noisy_bevy::*;
use raven_util::prelude::*;

pub mod prelude {
    pub use crate::{
        TerrainChunk,
        TerrainGenerator,
        TerrainPlugin,
        generate_mesh_from_heightfield,
        // water::*,
    };
}

use quadtree::QuadTree;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        if app.is_plugin_added::<EntropyPlugin<WyRand>>() {
            app.add_plugins(EntropyPlugin::<WyRand>::default());
        }

        app
        .add_plugins(TriplanarMaterialPlugin)
        .add_systems(
            Update,
            (
                //build_terrain,
                update_quad_tree,
                rebuild_terrain_on_change,
                build_terrain_chunk,
            )
                .chain(),
        )
        .add_systems(PostUpdate, (debug::draw_quadtree, debug::draw_chunk))
        .register_type::<TerrainGenerator>()
        .register_type::<TerrainChunk>();
    }
}

// TODO: remove this completely, and just store entity in left nodes
#[derive(Clone)]
pub struct TmpNode {
    position: Vec2,
    //_bounds: Aabb3d,
    dimensions: Vec2,
}

impl TmpNode {
    fn key(&self) -> String {
        format!(
            "{}/{} [{}]",
            self.position.x, self.position.y, self.dimensions.x
        )
    }
}

fn update_quad_tree(
    mut commands: Commands,
    camera_query: Single<&GlobalTransform, With<Camera>>,
    mut lookup: Local<HashMap<String, Entity>>,
    mut terrain_query: Query<(Entity, &TerrainGenerator, &mut TerrainChunks, &mut QuadTree)>,
) {
    let camera_transform = camera_query.into_inner();
    for (e, _t, mut chunks, mut tree) in terrain_query.iter_mut() {
        // update the quadtree
        tree.build(camera_transform.translation().xz());

        let mut new_chunks: ChunkMap = ChunkMap::new();
        for c in tree.get_children() {
            let child = TmpNode {
                position: c.bounds.center().xz(),
                //bounds: c.bounds,
                dimensions: (c.bounds.max - c.bounds.min).xz(),
            };
            new_chunks.insert(child.key(), child);
        }

        // these are entities that should already exist
        let mut add: ChunkMap = new_chunks
            .iter()
            .filter(|(k, _)| !chunks.0.contains_key(*k))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let remove = chunks
            .iter()
            .filter(|(k, _)| !new_chunks.contains_key(*k))
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>();

        // remove all chunks that are not in the new chunks
        for key in remove.iter() {
            if let Some(e) = lookup.remove(key) {
                commands.entity(e).despawn();
            }
        }

        // add all new chunks
        for (k, n) in &mut add {
            let e = commands
                .spawn((
                    ChildOf(e),
                    Name::new(format!("Terrain Chunk {}", n.key())),
                    TerrainChunk {
                        size: n.dimensions[0],
                    },
                    Transform::from_translation(Vec3::new(n.position.x, 0.0, n.position.y)),
                ))
                .id();
            lookup.insert(k.clone(), e);
        }

        // set the new chunk
        chunks.0 = new_chunks;
    }
}

pub type ChunkMap = HashMap<String, TmpNode, FixedHasher>;

#[derive(Component, Default, Deref, DerefMut)]
pub struct TerrainChunks(pub ChunkMap);

#[derive(Component, Reflect, Debug, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
#[require(Transform, InheritedVisibility, TerrainChunks, QuadTree)]
pub struct TerrainGenerator {
    #[inspector(min = 0.01, max = 1000.0, speed = 100.0, display = NumberDisplay::Slider)]
    pub height: f32,
    pub chunk_resolution: u32,
    /// Offset for the noise, can be used to shift the terrain
    pub noise_offset: Vec2,
    // Scale of the noise
    #[inspector(min = 0.01, max = 10000.0, speed = 1000.0, display = NumberDisplay::Slider)]
    pub noise_scale: f32,
    // Number of octaves to use for the noise, usually between 2 and 16
    #[inspector(min = 2, max = 16, speed = 1.0, display = NumberDisplay::Slider)]
    pub octaves: u32,
    // Lacunarity usually >= 2.0 (~ How much detail each octave increases)
    #[inspector(min = 2.0, max = 10.0, speed = 0.1, display = NumberDisplay::Slider)]
    pub lacunarity: f32,
    // Persistence usually ~ 0.5 (~ How much effect each octave has)
    #[inspector(min = 0.01, max = 1.0, speed = 0.1, display = NumberDisplay::Slider)]
    pub persistence: f32,
    // Exponent to apply to the final value
    #[inspector(min = 0.0, max = 10.0, speed = 1.0, display = NumberDisplay::Slider)]
    pub exp: f32,
}

impl Default for TerrainGenerator {
    fn default() -> Self {
        Self {
            height: 500.0,                     // Height scale for the terrain
            chunk_resolution: 64,              // Resolution of the terrain chunks
            noise_offset: Vec2::new(0.0, 0.0), // Offset for the noise, can be used to shift the terrain
            noise_scale: 2000.0,               // Not used, but can be used to scale the noise
            octaves: 4,
            lacunarity: 3.5,
            persistence: 0.33,
            exp: 2.2,
        }
    }
}

// fn build_terrain(
//     mut commands: Commands,
//     query: Query<(Entity, Option<&Children>, &TerrainGenerator), Changed<TerrainGenerator>>,
// ) {
//     for (e, children, t) in query.iter() {
//         // Remove old children
//         if let Some(children) = children {
//             for child in children.iter() {
//                 commands.entity(child).despawn();
//             }
//         }
//         let width = (t.size.x - 1) as f32 * t.chunk_size.x;
//         let height = (t.size.y - 1) as f32 * t.chunk_size.z;
//         let half_width = width * 0.5;
//         let half_height = height * 0.5;

//         for (i, x) in linspace(-half_width, half_width, t.size.x as usize).enumerate() {
//             for (j, y) in linspace(-half_height, half_height, t.size.y as usize).enumerate() {
//                 commands.spawn((
//                     ChildOf(e),
//                     Name::new(format!("Terrain Chunk {}, {}", i, j)),
//                     TerrainChunk {
//                         size: t.chunk_size.x,
//                     },
//                     Transform::from_xyz(x, 0.0, y),
//                 ));
//             }
//         }
//     }
// }

#[derive(Component, Reflect, Debug, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct TerrainChunk {
    size: f32,
}

#[derive(Component)]
pub struct HeightImage(pub Handle<Image>);

fn rebuild_terrain_on_change(
    mut commands: Commands,
    mut query: Query<(&Children, &mut TerrainChunks), Changed<TerrainGenerator>>,
) {
    // clear all children of the terrain generator
    for (children, mut chunks) in query.iter_mut() {
        // Remove old children
        for child in children.iter() {
            commands.entity(child).despawn();
        }
        chunks.0.clear();
    }
}

fn build_terrain_chunk(
    mut commands: Commands,
    query: Query<(Entity, &ChildOf, &Transform, &TerrainChunk), Changed<TerrainChunk>>,
    generator_query: Query<&TerrainGenerator>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (e, child_of, trans, chunk) in query.iter() {
        let Ok(generator) = generator_query.get(child_of.parent()) else {
            continue;
        };

        // scale everything to 0.0..1.0
        // TODO: reuse mesh, image, and heightfields, to avoid alloc, should all be same size
        let res = generator.chunk_resolution as usize;
        let mut heightfield = vec![vec![0.0; res]; res];
        for x in 0..res {
            for y in 0..res {
                heightfield[x][y] = {
                    // noise
                    let n = {
                        // Calculate normalized coordinates in 0..1 for this point in the chunk
                        let fx = x as f32 / (res - 1) as f32;
                        let fy = y as f32 / (res - 1) as f32;

                        // World position of this point
                        let world_x = trans.translation.x + (fx - 0.5) * chunk.size;
                        let world_y = trans.translation.z + (fy - 0.5) * chunk.size;

                        // Apply noise offset and scale
                        let nx = (world_x + generator.noise_offset.x) / generator.noise_scale;
                        let ny = (world_y + generator.noise_offset.y) / generator.noise_scale;

                        Vec2::new(nx, ny)
                    };

                    let mut total_noise_contribution = 0.0;
                    let mut current_frequency_multiplier = 1.0;
                    let mut current_amplitude = 1.0;
                    let mut sum_of_amplitudes = 0.0;

                    for _octave_num in 0..generator.octaves {
                        let raw_noise = simplex_noise_2d(n * current_frequency_multiplier);
                        total_noise_contribution += raw_noise * current_amplitude;
                        sum_of_amplitudes += current_amplitude;
                        current_frequency_multiplier *= generator.lacunarity;
                        current_amplitude *= generator.persistence;
                    }

                    let mut final_value = if sum_of_amplitudes > 0.0 {
                        (total_noise_contribution / sum_of_amplitudes) * 0.5 + 0.5
                    } else {
                        // Edge case, should not usually happen
                        0.5
                    };
                    final_value = final_value.powf(generator.exp);
                    //final_value -= 0.5 * generator.height_scale; // Center the heightfield around 0
                    final_value
                };
            }
        }

        let mut flat: Vec<[u8; 4]> = Vec::with_capacity(res * res);
        for y in 0..res {
            for x in 0..res {
                let h = (heightfield[x][y] * 255.0) as u8 as u8;
                flat.push([h, h, h, 255]);
            }
        }
        let image = Image::new_fill(
            Extent3d {
                width: res as u32,
                height: res as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            bytemuck::cast_slice(&flat),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        );

        let heightfield_handle = images.add(image);

        let scale = Vec3::new(chunk.size, generator.height, chunk.size);
        commands.entity(e).insert((
            Mesh3d(meshes.add(generate_mesh_from_heightfield(&heightfield, scale, true))),
            MeshMaterial3d(materials.add(StandardMaterial {
                //base_color: Color::srgb(1.0, 1.0, 1.0),
                base_color_texture: Some(heightfield_handle.clone()),
                ..default()
            })),
            //Collider::heightfield(heightfield, scale),
            //RigidBody::Static,
        ));
    }
}

/// Generates a mesh from a heightfield
/// Assumes the heightfield is square
#[allow(dead_code)]
pub fn generate_mesh_from_heightfield(
    heightfield: &Vec<Vec<f32>>,
    scale: Vec3,
    smooth: bool,
) -> Mesh {
    let size = heightfield.len();

    let num_vertices = size * size;
    let num_indices = (size - 1) * (size - 1) * 6;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);

    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
    let mut indices: Vec<u32> = Vec::with_capacity(num_indices);

    let half_size = (size - 1) as f32 / 2.0;
    for y in 0..size {
        for x in 0..size {
            let i = (y * size) + x;
            // find the position of the vertex and center, with height_multiplier
            let pos = [
                (x as f32 - half_size) * scale.x / (size - 1) as f32,
                (heightfield[x][y] * scale.y) as f32,
                (y as f32 - half_size) * scale.z / (size - 1) as f32,
            ];

            positions.push(pos);
            uvs.push([x as f32 / (size - 1) as f32, y as f32 / (size - 1) as f32]);

            if x < size - 1 && y < size - 1 {
                let a = i;
                let b = i + size;
                let c = i + size + 1;
                let d = i + 1;

                indices.push(a as u32);
                indices.push(b as u32);
                indices.push(c as u32);

                indices.push(c as u32);
                indices.push(d as u32);
                indices.push(a as u32);
            }
        }
    }

    // build our mesh
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_indices(Indices::U32(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    // compute normals and add positions
    match smooth {
        true => {
            let mut normals: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
            for y in 0..size {
                for x in 0..size {
                    let left_x = if x > 0 { x - 1 } else { x };
                    let right_x = if x < size - 1 { x + 1 } else { x };
                    let down_y = if y > 0 { y - 1 } else { y };
                    let up_y = if y < size - 1 { y + 1 } else { y };

                    let left: Vec3 = positions[y * size + left_x].into();
                    let right: Vec3 = positions[y * size + right_x].into();
                    let down: Vec3 = positions[down_y * size + x].into();
                    let up: Vec3 = positions[up_y * size + x].into();

                    let dx = right - left;
                    let dz = up - down;

                    let normal = dz.cross(dx).normalize_or_zero();
                    normals.push(normal.into());
                    // let pos: Vec3 = positions[(y * size + x) as usize].into();
                    // if x < size - 1 && y < size - 1 {
                    //     let pos_right: Vec3 = positions[(y * size + x + 1) as usize].into();
                    //     let pos_up: Vec3 = positions[((y + 1) * size + x) as usize].into();
                    //     let tangent1 = pos_right - pos;
                    //     let tangent2 = pos_up - pos;
                    //     let normal = tangent2.cross(tangent1);
                    //     normals.push(normal.normalize().into());
                    // } else {
                    //     normals.push(Vec3::Y.into());
                    // }
                }
            }
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        }
        false => {
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            mesh.duplicate_vertices();
            mesh.compute_flat_normals();
        }
    }

    mesh
}
