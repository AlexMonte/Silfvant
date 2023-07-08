use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub delta: Vec3,
}

impl Velocity {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            delta: Vec3::new(x, y, z),
        }
    }
}
