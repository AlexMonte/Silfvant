use std::ops::Add;

use bevy::{prelude::*, transform};

use crate::entity::component::position::*;
use crate::entity::component::velocity::Velocity;

fn update_component(d: &f32, r: &mut f32, c: &mut i32, time_delta_seconds: f32) {
    let mut steps = (*d * time_delta_seconds).abs().ceil() as i32;
    let step = *d * time_delta_seconds / steps as f32;
    while steps > 0 {
        *r += step;
        while *r > 1.0 {
            *r -= 1.0;
            *c += 1;
        }
        while *r < 0.0 {
            *r += 1.0;
            *c -= 1;
        }
        steps -= 1;
    }
}

pub fn movement(
    mut query: Query<(&mut Transform, &mut GridPosition, &mut GridRatio, &Velocity)>,
    time: Res<Time>,
) {
    for (mut transform, mut grid_position, mut grid_ratio, velocity) in query.iter_mut() {
        update_component(
            &velocity.delta.x,
            &mut grid_ratio.position.x,
            &mut grid_position.position.x,
            time.delta_seconds(),
        );
        update_component(
            &velocity.delta.y,
            &mut grid_ratio.position.y,
            &mut grid_position.position.y,
            time.delta_seconds(),
        );
        update_component(
            &velocity.delta.z,
            &mut grid_ratio.position.z,
            &mut grid_position.position.z,
            time.delta_seconds(),
        );

        transform.translation = grid_ratio.position.add(Vec3::new(
            grid_position.position.x as f32,
            grid_position.position.y as f32,
            grid_position.position.z as f32,
        ));
    }
}

pub fn update_transforms(mut query: Query<(&mut Transform, &mut GridPosition, &mut GridRatio)>) {
    for (transform, grid_position, grid_ratio) in query.iter_mut() {
        transform.translation.x = grid_position.x as f32 * grid_ratio.x;
        transform.translation.y = grid_position.y as f32 * grid_ratio.y;
        transform.translation.z = grid_position.z as f32 * grid_ratio.z;
    }
}
