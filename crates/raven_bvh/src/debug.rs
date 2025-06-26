use bevy::{
    color::palettes::tailwind, gizmos::{config::GizmoConfigGroup, AppGizmoBuilder}, prelude::*, render::view::RenderLayers
};

#[cfg(feature = "debug_draw")]
use crate::{
    BvhSystems,
    blas::{Blas, MeshBlas},
    tlas::Tlas,
};
#[cfg(feature = "debug_draw")]
use raven_util::prelude::*;


#[derive(Default)]
pub struct BvhDebugPlugin {
    //pub depth_bias: f32,
    pub render_layer: RenderLayers,
}

impl Plugin for BvhDebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_gizmo_config(
            BvhGizmos::default(),
            GizmoConfig {
                depth_bias: -0.01,
                render_layers: self.render_layer.clone(),
                ..Default::default()
            },
        ).init_resource::<BvhDebugMode>()
        .register_type::<BvhGizmos>();

        app.add_systems(
            PostUpdate,
            debug_gimos
                .after(BvhSystems::Update)
                .run_if(|store: Res<GizmoConfigStore>| store.config::<BvhGizmos>().0.enabled),
        );
    }
}

#[derive(Resource, Default, Debug)]
pub enum BvhDebugMode {
    #[default]
    Disabled,
    Bvhs,
    Tlas,
}

#[cfg(feature = "debug_draw")]
#[derive(Reflect, GizmoConfigGroup)]
pub struct BvhGizmos {
    pub blas_leaf: Option<Color>,
    pub blas_branch: Option<Color>,
    pub tlas_leaf: Option<Color>,
    pub tlas_branch: Option<Color>,
}

impl Default for BvhGizmos {
    fn default() -> Self {
        Self {
            blas_leaf: Some(tailwind::GREEN_500.into()),
            blas_branch: Some(tailwind::YELLOW_500.into()),            
            tlas_leaf: Some(tailwind::GREEN_500.into()),
            tlas_branch: Some(tailwind::YELLOW_500.into()),
        }
    }
}

#[cfg(feature = "debug_draw")]
pub fn debug_gimos(
    query: Query<(&MeshBlas, &GlobalTransform)>,
    bvhs: Res<Assets<Blas>>,
    bvh_debug: Res<BvhDebugMode>,
    tlases: Query<&Tlas>,
    store: Res<GizmoConfigStore>,
    mut gizmos: Gizmos,    
) {
    let config = store.config::<BvhGizmos>().1;
    match bvh_debug.as_ref() {
        BvhDebugMode::Disabled => (),
        BvhDebugMode::Bvhs => {
            for (b, global_trans) in query.iter() {
                let bvh = bvhs.get(&b.0).expect("Bvh not found");

                for node in &bvh.nodes {
                    let color = if node.is_leaf() {
                        config.blas_leaf
                    } else {
                        config.blas_branch
                    };
                    if let Some(color) = color {                        
                        gizmos.cuboid(aabb3d_transform(&node.aabb, global_trans), color);
                    }
                }
            }
        }
        BvhDebugMode::Tlas => {
            for tlas in tlases.iter() {
                for node in tlas.tlas_nodes.iter() {
                    let color = if node.is_leaf() {
                        config.tlas_leaf
                    } else {
                        config.tlas_branch
                    };
                    if let Some(color) = color {
                        gizmos.cuboid(aabb3d_global(&node.aabb), color);
                    }
                }
            }
        }
    }

    // gizmos.cuboid(
    //     aabb3d_global(&Aabb3d {
    //         min: Vec3A::splat(-1.0),
    //         max: Vec3A::splat(1.0),
    //     }),
    //     tailwind::RED_500,
    // );
}
