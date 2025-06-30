use bevy::{color::palettes::tailwind, prelude::*};
use raven_util::prelude::*;

use crate::{quadtree::QuadTree, TerrainChunk};

pub fn draw_quadtree(mut gizmos: Gizmos, query: Query<(&GlobalTransform, &QuadTree)>) {
    for (_trans, quadtree) in query.iter() {
        gizmos.cuboid(aabb3d_global(&quadtree.bounds), tailwind::GREEN_500);
        for c in quadtree.get_children() {
            // Draw the children of the quadtree
            gizmos.cuboid(aabb3d_global(&c.bounds), tailwind::RED_500);
        }
    }
}

pub fn draw_chunk(mut gizmos: Gizmos, query: Query<(&GlobalTransform, &TerrainChunk)>) {
    for (trans, _chunk) in query.iter() {
        gizmos.line(trans.translation(), Vec3::ZERO, tailwind::BLUE_500);
        // let bounds = Aabb3d::new(trans.translation(), Vec3::new(1.0, 1.0, 1.0));
        // gizmos.cuboid(aabb3d_global(&bounds), tailwind::GREEN_500);
    }
}