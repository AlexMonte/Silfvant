use super::{goal::Goal, world_state::WorldStateType};
use bevy::prelude::*;
use std::hash::Hash;

#[derive(Component)]
pub struct Values<'a, F, V>
where
    F: WorldStateType + Hash,
    V: WorldStateType,
{
    pub goal_set: Vec<Box<&'a dyn Goal<F, V>>>,
    pub current_goal: Option<Box<&'a dyn Goal<F, V>>>,
    pub blacklist_time: f32,
}

impl<'a, F, V> Values<'a, F, V>
where
    F: WorldStateType + Hash,
    V: WorldStateType,
{
    pub fn get_goal(&self) -> Option<&'a dyn Goal<F, V>> {
        self.current_goal
    }

    pub fn set_goal(&mut self, goal: &'a dyn Goal<F, V>) {
        self.current_goal = Some(goal);
    }
}
