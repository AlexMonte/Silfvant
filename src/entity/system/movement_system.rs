use std::ops::Add;

use bevy::prelude::*;

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

pub fn movement(mut query: Query<(&mut Transform, &mut Grid, &Velocity)>, time: Res<Time>) {
    for (mut transform, mut grid, velocity) in query.iter_mut() {
        update_component(
            &velocity.delta.x,
            &mut grid.ratio.x,
            &mut grid.position.x,
            time.delta_seconds(),
        );
        update_component(
            &velocity.delta.y,
            &mut grid.ratio.y,
            &mut grid.position.y,
            time.delta_seconds(),
        );
        update_component(
            &velocity.delta.z,
            &mut grid.ratio.z,
            &mut grid.position.z,
            time.delta_seconds(),
        );

        transform.translation = grid.ratio.add(Vec3::new(
            grid.position.x as f32,
            grid.position.y as f32,
            grid.position.z as f32,
        ));
    }
}

pub fn update_transforms(mut query: Query<(&mut Transform, &mut Grid)>) {
    for (mut transform, grid) in query.iter_mut() {
        transform.translation.x = grid.position.x as f32 * grid.ratio.x;
        transform.translation.y = grid.position.y as f32 * grid.ratio.y;
        transform.translation.z = grid.position.z as f32 * grid.ratio.z;
    }
}
