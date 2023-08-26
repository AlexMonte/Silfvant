use bevy::prelude::*;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::plugin::InputManagerSystem;
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};

use crate::engine::fs::Serializable;

// Player in game actions (movement, attack, etc.)
#[derive(Debug, Clone, Copy, Actionlike, Reflect, FromReflect, Default)]
pub(crate) enum PlayerAction {
    #[default]
    Move,
    Sprint,
    Dodge,
    Jump,
    Block,
    Attack1,
    Attack2,
    Skill,
    Interact,
    SpeedUpDialog,
}

// Camera actions (orbit, zoom, etc.)
#[derive(Debug, Clone, Actionlike, Reflect, FromReflect, Default)]
pub(crate) enum CameraAction {
    #[default]
    Orbit,
    Zoom,
}

// Action to interact with the UI and menus
#[derive(Debug, Clone, Actionlike, Reflect, FromReflect, Default)]
pub(crate) enum UiAction {
    #[default]
    TogglePause,
    ToggleInventory,
    ToggleMap,
    ToggleQuestLog,
    ToggleCharacterSheet,
    MoveCursor,
    Select,
    Cancel,
    Scroll,
}

pub(crate) fn create_player_action_input_manager_bundle() -> InputManagerBundle<PlayerAction> {
    InputManagerBundle {
        input_map: InputMap::new([
            (QwertyScanCode::Space, PlayerAction::Jump),
            (QwertyScanCode::LShift, PlayerAction::Sprint),
            (QwertyScanCode::E, PlayerAction::Interact),
        ])
        .insert(VirtualDPad::wasd(), PlayerAction::Move)
        .build(),
        ..default()
    }
}
