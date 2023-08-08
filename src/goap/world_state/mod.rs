use bevy::prelude::*;
use std::hash::Hash;
use std::ops::Add;

use bevy::utils::hashbrown::hash_map::*;

mod world_fact;
mod world_value;

pub use world_fact::*;
pub use world_value::*;

type WorldState = HashMap<WorldFact, WorldValue>;

impl WorldState {
    pub fn new() -> Self {
        let mut world_state: HashMap<String, WorldFact> = HashMap::new();
        Self(world_state)
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn has_conflicting_facts(&self, other: &WorldState) -> bool {
        for (fact, value) in self.0.iter() {
            if let Some(other_value) = other.try_to_get(fact) {
                if value != other_value && other_value != None {
                    return false;
                }
            }
        }
        false
    }

    pub fn conflicting_facts(self, other: &WorldState) -> HashMap<String, (WorldFact, WorldFact)> {
        let mut difference = HashMap::new();

        self.clear();
        for (fact, value) in self.iter() {
            if let Some(other_value) = other.try_to_get(fact) {
                if other != None && value != other_value {
                    difference.set(fact.clone(), (value.clone(), other_value.clone()));
                }
                difference
            }
        }
    }

    pub fn add_from_state(&mut self, other: &WorldState) {
        for (fact, value) in other.iter() {
            self.0.insert(fact.clone(), value.clone());
        }
    }

    pub fn try_to_get(&self, fact: &String) -> Option<&WorldFact> {
        self.0.get(fact)
    }

    pub fn try_to_get_mut(&mut self, fact: &String) -> Option<&mut WorldFact> {
        self.0.get_mut(fact)
    }
    pub fn set(&mut self, fact: String, value: WorldFact) {
        self.0.insert(fact, value);
    }

    pub fn remove(&mut self, fact: &String) -> Option<WorldFact> {
        self.0.remove(fact)
    }

    pub fn has_key(&self, fact: &String) -> bool {
        self.0.contains_key(fact)
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
    pub fn iter(&self) -> Iter {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut {
        self.0.iter_mut()
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            0: HashMap::default(),
        }
    }
}

impl Add for WorldState {
    type Output = Self;

    fn add(self, other: WorldState) -> WorldState {
        let mut result = self;
        for (fact, value) in other.iter() {
            result.set(fact.clone(), value.clone());
        }
        Self(result.0)
    }
}
