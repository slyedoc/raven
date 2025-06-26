
use bevy::{math::bounding::{Aabb3d, BoundingVolume}, prelude::*};



#[allow(dead_code)]
pub fn aabb3d_global(bounding: &Aabb3d) -> GlobalTransform {
    GlobalTransform::from(
        Transform::from_translation(bounding.center().into())
            .with_scale((bounding.max - bounding.min).into()),
    )
}

#[allow(dead_code)]
pub fn aabb3d_transform(bounding: &Aabb3d, transform: &GlobalTransform) -> GlobalTransform {
    *transform * aabb3d_global(bounding)
}