use bevy::prelude::*;
use bevy_gizmo::Gizmos;

fn draw_grid(gizmos: &mut bevy_gizmo::Gizmos) {
    const GRID_SIZE: i32 = 32;
    const CHUNKS: i32 = 1;
    const UNIT_DISTANCE: f32 = 4.0;

    for chunk in 0..CHUNKS {
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                for z in 0..GRID_SIZE {
                    let position = Vec3::new(
                        x as f32 * UNIT_DISTANCE,
                        y as f32 * UNIT_DISTANCE,
                        z as f32 * UNIT_DISTANCE,
                    );
                    gizmos.line(position, position + Vec3::new(4.0, 0.0, 0.0), Color::BLUE);
                    gizmos.line(position, position + Vec3::new(0.0, 4.0, 0.0), Color::GREEN);
                    gizmos.line(position, position + Vec3::new(0.0, 0.0, 4.0), Color::RED);
                }
            }
        }
    }
}
