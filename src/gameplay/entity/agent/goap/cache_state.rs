use crate::entity::npc::component::world_state::WorldState;
use bevy::prelude::*;
/// A cache of `WorldState` instances that can be recycled to reduce memory allocation.

#[derive(Resource, Eq, PartialEq, Clone)]
pub struct CacheState {
    cache: Vec<WorldState>,
}

impl CacheState {
    pub fn prepare_cache(&mut self, count: usize) {
        self.cache.resize(count, WorldState::new());
    }

    pub fn recycle(&mut self, state: WorldState) {
        self.cache.push(state);
    }

    pub fn instantiate(&mut self, old: Option<&WorldState>) -> WorldState {
        if let Some(cached_state) = self.cache.pop() {
            cached_state
        } else if Some(old_state) = old {
            old_state.clone()
        } else {
            WorldState::new()
        }
    }
}
