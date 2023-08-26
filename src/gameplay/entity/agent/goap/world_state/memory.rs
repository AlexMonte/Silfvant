use bevy::prelude::*;
use bevy::utils::hashbrown::hash_map::*;
use std::hash::Hash;

use crate::goap::world_state::*;

#[derive(Component)]
pub struct Memory {
    pub world_state: WorldState,
    pub sub_context: Vec<&WorldState>,
}

impl Memory {
    pub fn new() -> Self {
        Memory::default()
    }

    pub fn get(&self, fact: &WorldFact) -> Option<&WorldValue> {
        Some(self.world_state.get(fact))
    }

    pub fn get_mut(&mut self, fact: &WorldFact) -> Option<&mut WorldValue> {
        Some(self.world_state.get_mut(fact))
    }

    pub fn insert(&mut self, fact: WorldFact, value: WorldValue) {
        self.world_state.set(fact, value)
    }

    pub fn remove(&mut self, fact: &WorldFact) -> Option<WorldValue> {
        self.world_state.remove(fact)
    }

    pub fn contains(&self, fact: &WorldFact) -> bool {
        self.world_state.has_key(fact)
    }

    pub fn clear(&mut self) {
        self.world_state.clear()
    }

    pub fn iter(&self) -> Iter<'_, WorldFact, WorldValue> {
        self.world_state.iter()
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            world_state: WorldState::default(),
        }
    }
}

#[derive(Resource)]
pub struct WorldMemory {
    pub world_state: WorldState,
}

impl WorldMemory {
    pub fn new() -> Self {
        WorldMemory::default()
    }

    pub fn get(&self, fact: &WorldFact) -> Option<&WorldValue> {
        Some(self.world_state.get(fact))
    }

    pub fn get_mut(&mut self, fact: &WorldFact) -> Option<&mut WorldValue> {
        Some(self.world_state.get_mut(fact))
    }

    pub fn insert(&mut self, fact: WorldFact, value: WorldValue) {
        self.world_state.set(fact, value)
    }

    pub fn remove(&mut self, fact: &WorldFact) -> Option<WorldValue> {
        self.world_state.remove(fact)
    }

    pub fn contains(&self, fact: &WorldFact) -> bool {
        self.world_state.has_key(fact)
    }

    pub fn clear(&mut self) {
        self.world_state.clear()
    }

    pub fn iter(&self) -> Iter<'_, WorldFact, WorldValue> {
        self.world_state.iter()
    }
}

impl Default for WorldMemory {
    fn default() -> Self {
        Self {
            world_state: WorldState::default(),
        }
    }
}
