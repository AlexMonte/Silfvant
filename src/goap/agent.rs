use bevy::prelude::*;
use std::hash::Hash;
use std::sync::Arc;

use crate::goap::action::Action;
use crate::goap::goal::Goal;

use crate::entity::component::position::*;

pub enum AgentState {
    Idle,
    Running,
    Success,
    Failure,
}

#[derive(Default)]
pub struct AgentData<F, V>
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
    goal_set: Vec<Goal<F, V>>,
    action_set: Vec<Arc<dyn Action<F, V>>>,
    current_goal: Option<Goal<F, V>>,
    current_action: Option<Arc<dyn Action<F, V>>>,
}

pub struct AgentBuilder<F, V>
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
    goal_set: Vec<Goal<F, V>>,
    action_set: Vec<Arc<dyn Action<F, V>>>,
}

impl<F: Eq + PartialEq + Hash + Clone, V: Eq + PartialEq + Clone> AgentBuilder<F, V> {
    pub fn new() -> Self {
        Self {
            goal_set: Vec::new(),
            action_set: Vec::new(),
        }
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        commands.spawn(CPosition::new(0, 0, 0)).id()
    }
}
