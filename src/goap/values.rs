use super::goal::Goal;
use bevy::prelude::*;
use std::hash::Hash;

use super::world_state::WorldState;

#[derive(Component)]
pub struct Values<'a, F, V>
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
    pub goal_set: Vec<&'a Goal<F, V>>,
    pub current_goal: Option<&'a Goal<F, V>>,
    pub blacklist_time: f32,
}

impl<'a, F, V> Values<'a, F, V>
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
    pub fn get_goal(&self) -> Option<&'a Goal<F, V>> {
        self.current_goal
    }

    pub fn set_goal(&mut self, goal: &'a Goal<F, V>) {
        self.current_goal = Some(goal);
    }
}

#[cfg(test)]
mod tests {

    use crate::goap::{goal::GoalBuilder, values::Values, world_state::*};

    #[test]
    fn test_get_goal() {
        let desired_facts = WorldState::new(WorldFact::Safe, WorldValue::Is(true));

        let goal = GoalBuilder::new("Collect Resources")
            .desired_facts(desired_facts)
            .build();

        let values = Values {
            goal_set: vec![&goal],
            current_goal: Some(&goal),
            blacklist_time: 0.0,
        };
        assert_eq!(values.get_goal(), Some(&goal));
    }
}

impl<F, V> Iterator for WorldState<F, V>
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
    type Item = V;

    fn next(self: &mut WorldState<F, V>) -> Option<Self::Item> {
        // Implement the next method to iterate over the values in the WorldState
        unimplemented!()
    }
}
