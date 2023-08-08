use super::{goal::Goal, world_state::WorldStateType};
use bevy::prelude::*;
use std::hash::Hash;

#[derive(Component)]
pub struct Values<'a, String, WorldFact> {
    pub goal_set: Vec<Box<&'a dyn Goal>>,
    pub current_goal: Option<Box<&'a dyn Goal>>,
    pub blacklist_time: f32,
}

impl<'a, String, WorldFact> Values<'a, String, WorldFact> {
    pub fn get_goal(&self) -> Option<&'a dyn Goal> {
        self.current_goal
    }

    pub fn set_goal(&mut self, goal: &'a dyn Goal) {
        self.current_goal = Some(goal);
    }
}
