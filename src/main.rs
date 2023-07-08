//ignore unused code
#![allow(dead_code)]

use bevy::prelude::shape::UVSphere;
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

use std::env;

mod entity;
mod fsm;
mod goap;
mod utility;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;
const HEIGHT: f32 = 720.0;
const WIDTH: f32 = 1280.0;

fn main() {
    //if debug mode is enabled, set the RUST_BACKTRACE environment variable to 1
    if env::var("DEBUG").is_ok() {
        env::set_var("RUST_BACKTRACE", "1");
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(EditorPlugin::default())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Ground"),
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(500.0).into()),
            material: materials.add(Color::GREEN.into()),
            ..default()
        },
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn((
        Name::new("Camera"),
        MainCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 120.0, 200.0)
                .looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
            ..default()
        },
    ));

    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(UVSphere {
            radius: 1.0,
            sectors: 32,
            stacks: 16,
        })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    },));

    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(UVSphere {
            radius: 1.0,
            sectors: 32,
            stacks: 16,
        })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    },));
}
#[derive(Component)]
struct MainCamera;
