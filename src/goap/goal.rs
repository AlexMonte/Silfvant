use std::hash::Hash;

use super::world_state::{WorldState, WorldStateType};

pub trait Goal<F, V>: Send + Sync + 'static
where
    F: WorldStateType + Hash,
    V: WorldStateType,
{
    fn get_priority(&self, world_state: &WorldState<F, V>) -> f32;
    fn get_error_delay(&self) -> f32;
    fn get_desired_facts(&self) -> &WorldState<F, V>;
    fn get_name(&self) -> &'static str;
    fn is_goal_possible(&self, world_state: &WorldState<F, V>) -> bool;
    fn run(
        &self,
        world_state: &mut WorldState<F, V>,
        callback: &mut dyn FnMut(&mut WorldState<F, V>),
    ) -> bool;
    
}

//precalculations for the goal should be implemented as a system

macro_rules! define_goal {
    ($name:ident, $desired_facts:expr, $priority_fn:expr, $name_fn:expr) => {
        #[derive(Debug, Eq, PartialEq, Clone)]
        struct $name<F, V>
        where
            F: WorldStateType + Hash,
            V: WorldStateType,
        {
            // represents the name of the goal. This is used for debugging purposes
            pub name: &'static str,

            pub desired_facts: WorldState<F, V>,
        };

        impl<F, V> Goal<F, V> for $name
        where
            F: WorldStateType + Hash,
            V: WorldStateType,
        {
            fn evaluate_goal<F, V>(world_state: &WorldState<F, V>) -> f32
            where
                F: WorldStateType + Hash,
                V: WorldStateType,
            {
                $priority_fn(world_state)
            }

            fn get_desired_facts<F, V>() -> WorldState<F, V>
            where
                F: WorldStateType + Hash,
                V: WorldStateType,
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
