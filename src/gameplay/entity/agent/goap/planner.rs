use bevy::prelude::*;
use std::hash::Hash;

use super::ability::Ability;
use super::values::Values;
use super::world_state::*;

pub const MAX_ITERATIONS: i32 = 1000;
pub const MAX_NODES_TO_EXPAND: i32 = 10000;
