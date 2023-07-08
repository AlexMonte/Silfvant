pub mod component;
pub mod resource;

use crate::entity::system::MovementPlugin;

use crate::entity::component::position::*;
use crate::entity::component::velocity::Velocity;
//use crate::entity::component::world_state::*;
use bevy::prelude::*;
use pixelate_mesh::prelude::*;

pub struct NpcPlugin;

fn spawn_npc(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            // This cube will render at 64x64 pixels
            Pixelate::splat(64),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::WHITE.into()),
                ..default()
            },
        ))
        .insert(CPosition::new(0, 0, 0))
        .insert(RPosition::new(0.0, 0.0, 0.0))
        .insert(Velocity::new(0.0, 0.0, 0.0));
}
impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_npc).add_plugin(MovementPlugin);
    }
}
