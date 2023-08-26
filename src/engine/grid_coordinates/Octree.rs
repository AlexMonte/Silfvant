use bevy::prelude::*;
use bevy::render::primitive::AABB;

pub struct OctreeNode {
    pub bounds: AABB,
    pub children: Vec<dyn OctreeNode>,
    pub entities: Vec<Entity>,
    pub active: bool,
}

impl OctreeNode {
    pub fn subdivide(&mut self) {
        let half_size = self.bounds.half_size() / 2.0;
        let center = self.bounds.center();

        let mut new_children = Vec::new([Default::default(); 8]);

        for child in 0..8 {
            let offset = vec3::new(
                (child & 1) as f32 * half_size.x,
                ((child >> 1) & 1) as f32 * half_size.y,
                ((child >> 2) & 1) as f32 * half_size.z,
            );

            new_children.push(OctreeNode {
                bounds: AABB::from_half_extents(
                    center + Vec3::new(half_size.x, half_size.y, half_size.z),
                    half_size,
                ),
                children: Vec::new(),
                entities: Vec::new(),
                active: false,
            });
        }
        self.children = new_children;
    }
}
