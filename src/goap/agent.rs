use bevy::prelude::*;
use std::hash::Hash;
use std::sync::Arc;

use crate::goap::action::Action;
use crate::goap::goal::Goal;

use crate::entity::component::position::*;

use super::world_state::WorldStateType;

pub enum AgentState {
    Idle,
    Running,
    Success,
    Failure,
}

#[derive(Default)]
pub struct AgentData<'a, String, WorldFact> {
    goal_set: Vec<Box<dyn Goal>>,
    action_set: Vec<Arc<dyn Action>>,
    current_goal: Option<Box<&'a dyn Goal>>,
    current_action: Option<Arc<dyn Action>>,
}

pub struct AgentBuilder {
    goal_set: Vec<Box<dyn Goal>>,
    action_set: Vec<Arc<dyn Action>>,
}

impl AgentBuilder {
    pub fn new() -> Self {
        Self {
            goal_set: Vec::new(),
            action_set: Vec::new(),
        }
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        commands.spawn(Grid::new(0, 0, 0)).id()
    }
}
