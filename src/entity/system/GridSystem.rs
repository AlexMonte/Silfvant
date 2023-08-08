use bevy::prelude::*; 
use bevy::render::primitive::AABB;

struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_grid.system());
        app.add_system(draw_grid.system());
    }
}

// Function to set up the initial grid
fn setup_grid(commands: &mut Commands) {
    // Code to create chunks or grid entities

}

// Function to draw the grid using gizmos (only in debug mode)
#[cfg(feature = "debug_mode")]
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

#[derive(#[derive(Component)]
struct Chunk {
    pub bounds: AABB,

    //pub children: Vec<dyn Grid>,
    //pub entities: Vec<Entity>,
    //pub active: bool,
}