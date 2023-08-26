use super::world_state::{WorldState, WorldStateType};
use std::hash::Hash;

pub trait Goal: Send + Sync + 'static {
    fn get_priority(&self, world_state: &WorldState) -> f32;
    fn get_desired_state(&self) -> &WorldState;
    fn is_goal_possible(&self, world_state: &WorldState) -> bool;
}

//precalculations for the goal should be implemented as a system
macro_rules! define_goal {
    ($name:ident, $desired_facts:expr) => {
        #[derive(Debug, Eq, PartialEq, Clone)]
        pub struct $name
        where
            String: WorldStateType + Hash,
            WorldFact: WorldStateType,
        {
            // represents the name of the goal. This is used for debugging purposes
            pub name: &'static str,
            pub desired_facts: WorldState = $desired_facts,
        };
    }
}
macro_rules! define_goal {
    ($name:ident, $desired_facts:expr, $priority_fn:expr, $name_fn:expr) => {
        #[derive(Debug, Eq, PartialEq, Clone)]
        struct $name
        where
            String: WorldStateType + Hash,
            WorldFact: WorldStateType,
        {
            // represents the name of the goal. This is used for debugging purposes
            pub name: &'static str,

            pub desired_facts: WorldState,
        };

        impl Goal for $name
        where
            String: WorldStateType + Hash,
            WorldFact: WorldStateType,
        {
            fn evaluate_goal(world_state: &WorldState) -> f32
            where
                String: WorldStateType + Hash,
                WorldFact: WorldStateType,
            {
                $priority_fn(world_state)
            }

            fn get_desired_facts() -> WorldState
            where
                String: WorldStateType + Hash,
                WorldFact: WorldStateType,
            {
                $desired_facts
            }

            fn get_name() -> &'static str {
                $name_fn()
            }
        }
    };
}

// define_goal!(
//     Patrol,
//     WorldState::new(WorldFact::, WorldValue::None),
// )
