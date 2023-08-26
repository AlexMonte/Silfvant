//ignore unused code
//#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

use std::env;

pub mod config;

// Modules for the game
// The Engine module contains all the low level code for the game, like the physics, the rendering, the audio, etc.
pub mod engine;
// The Gameplay module contains all the high level code for the game, like the player, the enemies, the items, etc.
pub mod gameplay;
// The Development module contains all the code that is only used during development, like the debug UI, the console, etc.
#[cfg(feature = "dev")]
pub mod dev;

use crate::engine::EnginePlugin;
use crate::gameplay::GameplayPlugin;

#[cfg(feature = "dev")]
use crate::dev::DevPlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    /// During the loading State the loading_plugin will load our assets
    #[default]
    Loading,
    /// During this State the actual game logic is executed
    Playing,
    /// Here the menu is drawn and waiting for player interaction
    Menu,
}
struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state::<GameState>()
            .add_plugin(EnginePlugin)
            .add_plugin(GameplayPlugin);

        #[cfg(feature = "dev")]
        app.add_plugin(DevPlugin);
    }
}
