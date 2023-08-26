use bevy::prelude::*;
use std::hash::Hash;
use std::ops::Add;

use bevy::utils::hashbrown::hash_map::*;

mod world_fact;
mod world_value;

pub use world_fact::*;
pub use world_value::*;

type WorldState = HashMap<WorldFact, WorldValue>;
pub trait WorldStateTrait:
    Add<WorldState, Output = WorldState> + Eq + PartialEq + Hash + Clone + Default
{
    fn has_conflicts(&self, other: &Self) -> bool;
    fn has_similarities(&self, other: &Self) -> bool;

    fn get_conflicts(&self, other: &Self) -> Vec<WorldFact>;
    fn get_differences(&self, other: &Self) -> Vec<WorldFact>;
    fn get_similarities(&self, other: &Self) -> Vec<WorldFact>;
}

impl WorldStateTrait for WorldState {
    fn has_conflicts(&self, other: &Self) -> bool {
        self.iter()
            .filter(|(fact, _)| other.contains_key(fact))
            .any(|(fact, value)| other.get(fact) != Some(value))
    }

    fn has_similarities(&self, other: &Self) -> bool {
        self.iter()
            .filter(|(fact, _)| other.contains_key(fact))
            .any(|(fact, value)| other.get(fact) == Some(value))
    }

    fn get_conflicts(&self, other: &Self) -> Vec<WorldFact> {
        self.iter()
            .filter(|(fact, _)| other.contains_key(fact))
            .filter(|(fact, value)| other.get(fact) != Some(value))
            .map(|(fact, _)| fact.clone())
            .collect()
    }

    fn get_differences(&self, other: &Self) -> Vec<WorldFact> {
        self.iter()
            .filter(|(fact, _)| !other.contains_key(fact))
            .map(|(fact, _)| fact.clone())
            .collect()
    }

    fn get_similarities(&self, other: &Self) -> Vec<WorldFact> {
        self.iter()
            .filter(|(fact, _)| other.contains_key(fact))
            .filter(|(fact, value)| other.get(fact) == Some(value))
            .map(|(fact, _)| fact.clone())
            .collect()
    }
}

impl Eq for WorldState {
    fn eq(&self, other: &Self) -> bool {
        self.iter()
            .filter(|(fact, _)| other.contains_key(fact))
            .all(|(fact, value)| other.get(fact) == Some(value))
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}
