mod friction_system;
mod movement_system;

use bevy::prelude::*;

use super::system::friction_system::friction;
use super::system::movement_system::movement;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement).add_system(friction);
    }
}
