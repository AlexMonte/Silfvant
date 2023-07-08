use crate::goap::world_state::WorldState;
use bevy::prelude::*;
use bevy::utils::hashbrown::hash_map::*;
use std::hash::Hash;

#[derive(Component)]
pub struct Memory<F, V>
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
    pub world_state: WorldState<F, V>,
}

impl<F, V> Memory<F, V>
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
    pub fn new() -> Self {
        Memory::default()
    }

    pub fn get(&self, fact: &F) -> Option<&V> {
        Some(self.world_state.get(fact))
    }

    pub fn get_mut(&mut self, fact: &F) -> Option<&mut V> {
        Some(self.world_state.get_mut(fact))
    }

    pub fn insert(&mut self, fact: F, value: V) {
        self.world_state.set(fact, value)
    }

    pub fn remove(&mut self, fact: &F) -> Option<V> {
        self.world_state.remove(fact)
    }

    pub fn contains(&self, fact: &F) -> bool {
        self.world_state.has_key(fact)
    }

    pub fn clear(&mut self) {
        self.world_state.clear()
    }

    pub fn iter(&self) -> Iter<'_, F, V> {
        self.world_state.iter()
    }
}

impl<F, V> Default for Memory<F, V>
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
    fn default() -> Self {
        Self {
            world_state: WorldState::default(),
        }
    }
}

#[derive(Resource)]
pub struct WorldMemory<F, V>
where
    F: Eq + PartialEq + Hash + Clone + Send + Sync,
    V: Eq + PartialEq + Clone + Send + Sync,
{
    pub world_state: WorldState<F, V>,
}

impl<F, V> WorldMemory<F, V>
where
    F: Eq + PartialEq + Hash + Clone + Send + Sync,
    V: Eq + PartialEq + Clone + Send + Sync,
{
    pub fn new() -> Self {
        WorldMemory::default()
    }

    pub fn get(&self, fact: &F) -> Option<&V> {
        Some(self.world_state.get(fact))
    }

    pub fn get_mut(&mut self, fact: &F) -> Option<&mut V> {
        Some(self.world_state.get_mut(fact))
    }

    pub fn insert(&mut self, fact: F, value: V) {
        self.world_state.set(fact, value)
    }

    pub fn remove(&mut self, fact: &F) -> Option<V> {
        self.world_state.remove(fact)
    }

    pub fn contains(&self, fact: &F) -> bool {
        self.world_state.has_key(fact)
    }

    pub fn clear(&mut self) {
        self.world_state.clear()
    }

    pub fn iter(&self) -> Iter<'_, F, V> {
        self.world_state.iter()
    }
}

impl<F, V> Default for WorldMemory<F, V>
where
    F: Eq + PartialEq + Hash + Clone + Send + Sync,
    V: Eq + PartialEq + Clone + Send + Sync,
{
    fn default() -> Self {
        Self {
            world_state: WorldState::default(),
        }
    }
}
