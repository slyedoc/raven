use bevy::math::bounding::{Aabb3d, BoundingVolume};
use bevy::prelude::*;
use bevy::math::{Vec3};

#[derive(Debug)]
pub struct QuadNode {
    pub bounds: Aabb3d,
    pub children: Vec<QuadNode>,
    pub center: Vec2,
    pub size: Vec2,
}

impl QuadNode {
    fn new(min: Vec3, max: Vec3) -> Self {

        let bounds = Aabb3d {
            min: min.into(),
             max: max.into(),
        };
        let center = bounds.center().xz();        
        let size = (bounds.max - bounds.min).xz(); // Size is the full width and depth of the node
        
        Self {
            bounds,
            children: Vec::new(),
            center,
            size,
        }
    }

    pub fn build(&mut self, pos: Vec2, min_node_size: f32) {
        let dist_to_cam = self.center.distance(pos);
        if dist_to_cam < self.size.x && self.size.x > min_node_size {            
            self.children = self.create_children();                    
            for child in &mut self.children {                
                child.build(pos, min_node_size);                
            }
        }
    }

    pub fn create_children(&self) -> Vec<QuadNode> {
        let min = self.bounds.min;
        let max = self.bounds.max;
        let center = self.bounds.center();

        let (x0, x1, x2) = (min.x, center.x, max.x);
        let (z0, z1, z2) = (min.z, center.z, max.z);
        let y0 = min.y;
        let y1 = max.y;

        vec![
            // Bottom-left
            QuadNode::new(Vec3::new(x0, y0, z0), Vec3::new(x1, y1, z1)),
            // Bottom-right
            QuadNode::new(Vec3::new(x1, y0, z0), Vec3::new(x2, y1, z1)),
            // Top-left
            QuadNode::new(Vec3::new(x0, y0, z1), Vec3::new(x1, y1, z2)),
            // Top-right
            QuadNode::new(Vec3::new(x1, y0, z1), Vec3::new(x2, y1, z2)),
        ]
    }
}

#[derive(Component, Debug)]
pub struct QuadTree {
    pub bounds: Aabb3d,
    pub min_node_size: f32,    
    pub root: QuadNode,
}

impl Default for QuadTree {
    fn default() -> Self {
        let min = Vec3::new(-32000.0, 0.0, -32000.0);
        let max = Vec3::new(32000.0, 0.0, 32000.0);   
        Self::new(min, max)
    }
}

impl QuadTree {

    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self {
            min_node_size: 500.0,
            root: QuadNode::new(min, max),
            bounds: Aabb3d {
                min: min.into(),
                max: max.into(),
            },
        }   
    }

    pub fn get_children(&self) -> Vec<&QuadNode> {
        let mut out = Vec::new();
        self.get_children_recursive(&self.root, &mut out);
        out
    }

    pub fn build(&mut self, camera_pos: Vec2) {
        self.root = QuadNode::new(self.root.bounds.min.into(), self.root.bounds.max.into());
        self.root.build(camera_pos, self.min_node_size);
    }

    fn get_children_recursive<'a>(&'a self, node: &'a QuadNode, out: &mut Vec<&'a QuadNode>) {
        if node.children.is_empty() {
            out.push(node);
        } else {
            for child in &node.children {
                self.get_children_recursive(child, out);
            }
        }
    }



    
}
