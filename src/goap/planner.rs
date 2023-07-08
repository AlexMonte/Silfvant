use bevy::prelude::*;
use std::hash::Hash;

use super::ability::Ability;
use super::memory::*;
use super::values::Values;

pub const MaxIterations: i32 = 1000;
pub const MaxNodesToExpand: i32 = 10000;
