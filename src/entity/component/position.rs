use bevy::prelude::*;

#[derive(Component)]
pub struct GridPosition(IVec3);

impl GridPosition {
    #[inline]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self(IVec3::new(x, y, z))
    }
}

#[derive(Component)]
pub struct GridRatio(Vec3);

impl GridRatio {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z))
    }
}
