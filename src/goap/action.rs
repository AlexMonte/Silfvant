use bevy::prelude::*;
use std::collections::LinkedList;
use std::hash::Hash;
use std::sync::Arc;

use crate::goap::world_state::WorldState;

use super::world_state::WorldStateType;

pub struct ActionStackData<F, V>
where
    F: WorldStateType + Hash,
    V: WorldStateType,
{
    pub current_state: WorldState<F, V>,
    pub goal_state: WorldState<F, V>,
    pub next: Arc<dyn Action<F, V>>,
    pub settings: WorldState<F, V>,
}

pub trait Action<F, V>: Send + Sync
where
    F: WorldStateType + Hash,
    V: WorldStateType,
{
    // this should return current's action calculated parameter, will be added to the run method
    // userful for dynamic actions, for example a GoTo action can save some informations (wanted position)
    // while being chosen from the planner, we save this information and give it back when we run the method
    // most of actions would return a single item list, but more complex could return many items

    fn get_settings(&self, stack_data: ActionStackData<F, V>) -> LinkedList<WorldState<F, V>>;
    fn run(
        &self,
        previous_action: Box<dyn Action<F, V>>,
        next_action: Box<dyn Action<F, V>>,
        settings: WorldState<F, V>,
        goal_state: WorldState<F, V>,
        done: Arc<dyn Action<F, V>>,
        fail: Arc<dyn Action<F, V>>,
    );
    // Called when the action has been added inside a running Plan
    fn plan_enter(
        &self,
        previous_action: Arc<dyn Action<F, V>>,
        next_action: Arc<dyn Action<F, V>>,
        settings: WorldState<F, V>,
        goal_atate: WorldState<F, V>,
    );
    // Called when the plan, which had this action, has either failed or completed
    fn plan_exit(
        &self,
        previous_action: Arc<dyn Action<F, V>>,
        next_action: Arc<dyn Action<F, V>>,
        settings: WorldState<F, V>,
        goal_atate: WorldState<F, V>,
    );
    fn exit(&self, next_action: Arc<dyn Action<F, V>>);
    fn get_name(&self) -> String;
    fn is_active(&self) -> bool;
    fn is_interruptable(&self) -> bool;
    fn ask_for_interruption(&self);
    // MUST BE IMPLEMENTED AS THREAD SAFE
    fn get_preconditions(&self, stack_data: ActionStackData<F, V>) -> WorldState<F, V>;
    fn get_effects(&self, stack_data: ActionStackData<F, V>) -> WorldState<F, V>;
    fn check_procedural_condition(&self, stack_data: ActionStackData<F, V>) -> bool;
    fn get_cost(&self, stack_data: ActionStackData<F, V>) -> f32;
    // DO NOT CHANGE RUNTIME ACTION VARIABLES, precalculation can be runned many times even while an action is running
    fn is_affordable(&self, data: WorldState<F, V>) -> bool;

    fn to_string(&self, stack_data: ActionStackData<F, V>) -> String;
}

pub struct ActionState<F, V>
where
    F: WorldStateType + Hash,
    V: WorldStateType,
{
    pub action: Arc<dyn Action<F, V>>,
    pub settings: WorldState<F, V>,
}
