use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::sync::{Arc, Mutex};

use super::world_state::WorldState;

pub trait GoalBehavior<F, V>: Debug + Send + Sync + 'static
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
    fn get_priority(&self, world_state: &WorldState<F, V>) -> f32;
    fn get_error_delay(&self) -> f32;
    fn get_desired_facts(&self) -> &WorldState<F, V>;
    fn get_name(&self) -> &'static str;
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Goal<F, V>
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
    pub name: &'static str,
    pub desired_facts: WorldState<F, V>,
    pub behavior: Box<dyn GoalBehavior<F, V>>,
}

pub struct GoalBuilder<F, V>
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
    name: &'static str,
    desired_facts: WorldState<F, V>,
    behavior: Box<dyn GoalBehavior<F, V>>,
}

impl<F, V> GoalBuilder<F, V>
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            desired_facts: WorldState::default(),
            behavior: None,
        }
    }

    pub fn desired_facts(mut self, desired_facts: WorldState<F, V>) -> Self {
        self.desired_facts = desired_facts;
        self
    }

    pub fn behavior(
        mut self,
        behavior: Arc<Mutex<(dyn GoalBehavior<F, V> + Send + 'static)>>,
    ) -> Self {
        self.behavior = Some(behavior);
        self
    }

    pub fn build(self) -> Goal<F, V> {
        Goal {
            name: self.name,
            desired_facts: self.desired_facts,
            behavior: self.behavior.expect("GoalBehavior must be set"),
        }
    }
}

//test
#[cfg(test)]
mod test {
    //write unit test for the goal module
    use super::*;
    //use crate::goap::WorldFact::*;

    #[derive(Debug, Eq, PartialEq, Hash, Clone)]
    enum Fact {
        A,
        B,
        C,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    enum Value {
        True,
        False,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct TestGoalBehavior {}

    impl GoalBehavior<Fact, Value> for TestGoalBehavior {
        fn get_priority(&self, world_state: &WorldState<Fact, Value>) -> f32 {
            0.0
        }

        fn get_error_delay(&self) -> f32 {
            0.0
        }

        fn get_desired_facts(&self) -> &WorldState<Fact, Value> {
            &WorldState::default()
        }

        fn get_name(&self) -> &'static str {
            "TestGoalBehavior"
        }
    }
}
