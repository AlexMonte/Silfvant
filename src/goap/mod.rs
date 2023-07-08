mod ability;
mod action;
mod agent;
mod goal;
mod memory;
mod plan;
mod planner;
mod senses;
mod values;

mod world_state;

use ability::Ability;
use bevy::prelude::*;
use memory::Memory;
use std::hash::Hash;
// Method for triggering and executing the entity's Instincts
pub fn FightOrFlightResponse<'ability, 'memory, F, V>(
    query: Query<(&'ability Ability<F, V>, &'memory Memory<F, V>)>,
) where
    F: 'static + Eq + PartialEq + Hash + Clone + Send + Sync,
    V: 'static + Eq + PartialEq + Clone + Send + Sync,
{
    for (ability, memory) in query.iter() {
       
    }
}
