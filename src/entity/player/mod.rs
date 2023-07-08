use bevy::prelude::*;

use super::position::CPosition;
use super::position::RPosition;
use super::velocity::Velocity;

#[derive(Component)]
pub struct Player;

pub fn spanwn_player(commands: &mut Commands) {
    commands.spawn((
        Player,
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        RPosition::new(0.0, 0.0, 0.0),
        CPosition::new(0, 0, 0),
        Velocity::new(0.0, 0.0, 0.0),
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
}
