use bevy::prelude::*;

use super::world_state::*;

use super::memory::Memory;

pub trait Action: Component {
    fn heuristic(&self, memory: &Memory<Self::WorldStateType>) -> i32;
    fn cost(&self, memory: &Memory<Self::WorldStateType>) -> i32;
}
