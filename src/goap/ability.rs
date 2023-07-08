use bevy::prelude::*;
use std::hash::Hash;
use std::sync::Arc;

use crate::goap::action::Action;
use crate::goap::memory::Memory;

#[derive(Component, Clone)]
pub struct Ability<F, V> {
    action_set: Vec<Arc<dyn Action<F, V>>>,
    pub current_action: Option<Arc<dyn Action<F, V>>>,
}

impl<F, V> Ability<F, V>
where
    F: Eq + PartialEq + Hash + Clone + Reflect,
    V: Eq + PartialEq + Clone + Reflect,
{
    pub fn new() -> Self {
        Self {
            action_set: Vec::new(),
            current_action: None,
        }
    }

    pub fn add_action(&mut self, action: Arc<dyn Action<F, V>>) {
        self.action_set.push(action);
    }

    pub fn set_action(&mut self, action: Arc<dyn Action<F, V>>) {
        self.current_action = Some(action);
    }

    pub fn affordances(&self, memory: Memory<F, V>) -> Vec<Arc<dyn Action<F, V>>> {
        let mut affordances = Vec::new();
        for action in &self.action_set {
            if action.is_affordable(memory.world_state.clone()) {
                affordances.push(action.clone());
            }
        }
        affordances
    }
}
