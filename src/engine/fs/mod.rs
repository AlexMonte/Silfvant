use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, iter};

pub(crate) mod asset_loading;
pub(crate) mod audio;
pub(crate) mod config;
pub(crate) mod game_state_serialization;
pub(crate) mod level_serialization;

pub use crate::{
    file_system_interaction::asset_loading::loading_plugin,
    file_system_interaction::audio::internal_audio_plugin,
    file_system_interaction::game_state_serialization::game_state_serialization_plugin,
    file_system_interaction::level_serialization::level_serialization_plugin,
};

use bevy::prelude::*;
use engine::*;

// Trait for serializing and deserializing objects to and from strings.
//
pub(crate) trait Serializable {
    fn serialize(&self) -> Result<String, Error>;
    fn deserialize(&self) -> Result<Self, Error>
    where
        Self: Sized;
}

pub(crate) struct FileHandler;

impl FileHandler {
    pub(crate) fn save<T: Serializable>(path: &Path, object: &T) -> Result<()> {
        let data: <Result<String, _> as Try>::Output = object.serialize()?;
        fs::write(path, data)?;
        Ok(())
    }

    pub(crate) fn load<T: Serializable>(path: &Path) -> Result<T> {
        let data: String = fs::read_to_string(path)?;
        T::deserialize(&data)
    }
}

/// Handles loading and saving of levels and save states to disk.
/// Split into the following sub-plugins:
/// - [`loading_plugin`] handles loading of assets.
/// - [`game_state_serialization_plugin`] handles saving and loading of game states.
/// - [`level_serialization_plugin`] handles saving and loading of levels.
/// - [`internal_audio_plugin`]: Handles audio initialization
pub(crate) fn file_system_interaction_plugin(app: &mut App) {
    app.add_fn_plugin(loading_plugin)
        .add_fn_plugin(game_state_serialization_plugin)
        .add_fn_plugin(level_serialization_plugin)
        .add_fn_plugin(internal_audio_plugin);
}
