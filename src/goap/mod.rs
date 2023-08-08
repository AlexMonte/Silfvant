mod agent;
mod goal;
mod plan;
mod planner;
mod senses;
mod values;
mod world_state;

mod world_state;

use ability::Ability;
use bevy::prelude::*;
use std::hash::Hash;
use world_state::Memory;

use self::world_state::WorldStateType;
// Method for triggering and executing the entity's Instincts
pub fn FightOrFlightResponse<'ability, 'memory, String, WorldFact>(
    query: Query<(&'ability Ability, &'memory Memory)>,
) {
    for (ability, memory) in query.iter() {}
}
