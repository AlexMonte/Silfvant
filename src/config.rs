use bevy::window::WindowMode;

pub(crate) const GAME_TITLE: &str = "Rust Gamebase";

// Defines the amount of time that should elapse between each physics step.
pub(crate) const TIME_STEP: f32 = 1.0 / 60.0;

pub(crate) const WINDOW_WIDTH: f32 = 1280.0;
pub(crate) const WINDOW_HEIGHT: f32 = 720.0;
pub(crate) const WINDOW_WIDTH_HALF: f32 = WINDOW_WIDTH / 2.;
pub(crate) const WINDOW_HEIGHT_HALF: f32 = WINDOW_HEIGHT / 2.;

pub(crate) const WINDOW_MODE: WindowMode = WindowMode::BorderlessFullscreen;
