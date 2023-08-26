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

#[derive(Component)]
struct Chunk {}
