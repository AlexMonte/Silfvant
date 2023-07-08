use bevy::prelude::*;

#[derive(Component)]
pub struct Grid {
    pub position: IVec3,
    pub ratio: Vec3,
}

impl Grid {
    #[inline]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            position: IVec3::new(x, y, z),
            ratio: Vec3::new(0.5, 0.0, 0.5),
        }
    }
}
