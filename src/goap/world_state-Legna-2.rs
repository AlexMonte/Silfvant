use bevy::log;
use bevy::prelude::Reflect;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::ops::Add;

use bevy::utils::hashbrown::hash_map::*;

#[derive(Eq, Hash, PartialEq, Clone, Reflect, Debug)]
pub enum WorldFact {
    None,
    Safe,
    CollectResource,
    Enemy,
}
#[derive(Eq, Hash, PartialEq, Clone, Reflect, Debug)]
pub enum WorldValue {
    None,
    Is(bool),
    Entity(u32),
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct WorldState<F: Eq + PartialEq + Clone, V: Eq + PartialEq + Clone>(HashMap<F, V>);

impl<F, V> WorldState<F, V>
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
    HashMap<F, V>: Hash,
{
    pub fn new(key: F, value: V) -> Self {
        let mut world_state = HashMap::new();
        world_state.insert(key, value);
        Self(world_state)
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn has_conflicting_facts(&self, other: WorldState<F, V>) -> bool {
        for (fact, value) in self.0.iter() {
            if let Some(other_value) = other.try_to_get(fact) {
                if value != other_value {
                    println!("Conflict: {0} with {1} and {2}", fact, value, other_value);
                    return false;
                }
            }
        }
        false
    }

    pub fn conflicting_facts(
        &mut self,
        other: &Self,
        mut difference: Option<&mut Self>,
        predicate: Option<Box<dyn Fn(F, V) -> bool>>,
    ) -> i32 {
        let mut count = 0;
        let buffer = self.clone();

        self.clear();

        for (fact, value) in buffer.iter() {
            if let Some(other_value) = other.try_to_get(fact) {
                if value != other_value && predicate.is_none()
                    || predicate.as_ref().unwrap()(fact.clone(), value.clone())
                {
                    println!("Conflict: {0} with {1} and {2}", fact, value, other_value);
                    if let Some(difference) = difference.as_mut() {
                        difference.set(fact.clone(), value.clone());
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn add_from_state(&mut self, other: &Self) {
        for (fact, value) in other.iter() {
            self.0.insert(fact.clone(), value.clone());
        }
    }

    pub fn get(&self, fact: &F) -> &V {
        self.0.get(fact).unwrap()
    }
    pub fn get_mut(&mut self, fact: &F) -> &mut V {
        self.0.get_mut(fact).unwrap()
    }
    pub fn try_to_get(&self, fact: &F) -> Option<&V> {
        self.0.get(fact)
    }

    pub fn try_to_get_mut(&mut self, fact: &F) -> Option<&mut V> {
        self.0.get_mut(fact)
    }
    pub fn set(&mut self, fact: F, value: V) {
        self.0.insert(fact, value);
    }

    pub fn remove(&mut self, fact: &F) -> Option<V> {
        self.0.remove(fact)
    }

    pub fn has_key(&self, fact: &F) -> bool {
        self.0.contains_key(fact)
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
    pub fn iter(&self) -> Iter<F, V> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<F, V> {
        self.0.iter_mut()
    }
}

impl<F, V> Default for WorldState<F, V>
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
    fn default() -> Self {
        Self {
            0: HashMap::default(),
        }
    }
}

impl<F, V> Add for WorldState<F, V>
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
    type Output = Self;

    fn add(self, other: WorldState<F, V>) -> WorldState<F, V> {
        let mut result = self;
        for (fact, value) in other.iter() {
            result.set(fact.clone(), value.clone());
        }
        Self(result.0)
    }
}
