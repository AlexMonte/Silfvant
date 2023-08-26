use bevy::prelude::*;
use std::hash::Hash;
use std::sync::Arc;

use crate::goap::action::Action;
use crate::goap::memory::Memory;

use super::world_state::WorldStateType;

#[derive(Component, Clone)]
pub struct Ability {
    action_set: Vec<Arc<dyn Action>>,
    pub current_action: Option<Arc<dyn Action>>,
}

impl Ability {
    pub fn new() -> Self {
        Self {
            action_set: Vec::new(),
            current_action: None,
        }
    }

    pub fn add_action(&mut self, action: Arc<dyn Action>) {
        self.action_set.push(action);
    }

    pub fn set_action(&mut self, action: Arc<dyn Action>) {
        self.current_action = Some(action);
    }

    pub fn affordances(&self, memory: Memory) -> Vec<Arc<dyn Action>> {
        self.action_set
            .iter()
            .filter(|action| action.is_affordable(&memory))
            .cloned()
            .collect()
    }
}
