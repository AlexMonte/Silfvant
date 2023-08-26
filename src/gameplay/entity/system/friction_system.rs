use bevy::prelude::*;

use crate::entity::component::velocity::Velocity;
pub const FRICTION: Vec3 = Vec3::new(0.82, 0.82, 0.82);
pub const CLEARTHRESHOLD: f32 = 0.0005;

pub fn friction(mut query: Query<&mut Velocity>) {
    for mut velocity in query.iter_mut() {
        velocity.delta *= FRICTION;
        if velocity.delta.abs().abs_diff_eq(Vec3::ZERO, CLEARTHRESHOLD) {
            velocity.delta = Vec3::ZERO;
        }
    }
}
