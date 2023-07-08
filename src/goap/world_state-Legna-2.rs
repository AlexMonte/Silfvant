use bevy::prelude::Reflect;
use bevy::reflect::FromReflect;
use std::hash::Hash;
use std::ops::Add;

use bevy::utils::hashbrown::hash_map::*;
pub trait WorldStateType: Eq + PartialEq + Clone + Reflect + Send + Sync {}

#[derive(Eq, Hash, PartialEq, Clone, Reflect, FromReflect, Debug)]
pub enum WorldFact {
    None,
    Safe,
    CollectResource,
    Enemy,
}
impl WorldStateType for WorldFact {}
#[derive(Eq, Hash, PartialEq, Clone, Reflect, FromReflect, Debug)]
pub enum WorldValue {
    None,
    Is(bool),
    Entity(u32),
}
impl WorldStateType for WorldValue {}

#[derive(Eq, PartialEq, Clone, Reflect, Debug)]
pub struct WorldState<F, V>
where
    F: WorldStateType + Hash,
    V: WorldStateType,
{
    pub state: HashMap<F, V>,
}

impl<F, V> WorldState<F, V>
where
    F: WorldStateType + Hash,
    V: WorldStateType,
    HashMap<F, V>: Hash,
{
    pub fn new(key: F, value: V) -> Self {
        let mut state = HashMap::new();
        state.insert(key, value);
        WorldState { state }
    }

    pub fn count(&self) -> usize {
        self.state.len()
    }

    pub fn has_conflicting_facts(&self, other: WorldState<F, V>) -> bool {
        for (fact, value) in self.state.iter() {
            if let Some(other_value) = other.try_to_get(fact) {
                if value != other_value {
                    //log!("Conflict: {0} with {1} and {2}", fact, value, other_value);
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
                    //log!("Conflict: {0} with {1} and {2}", fact, value, other_value);
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
            self.state.insert(fact.clone(), value.clone());
        }
    }

    pub fn get(&self, fact: &F) -> &V {
        self.state.get(fact).unwrap()
    }
    pub fn get_mut(&mut self, fact: &F) -> &mut V {
        self.state.get_mut(fact).unwrap()
    }
    pub fn try_to_get(&self, fact: &F) -> Option<&V> {
        self.state.get(fact)
    }

    pub fn try_to_get_mut(&mut self, fact: &F) -> Option<&mut V> {
        self.state.get_mut(fact)
    }
    pub fn set(&mut self, fact: F, value: V) {
        self.state.insert(fact, value);
    }

    pub fn remove(&mut self, fact: &F) -> Option<V> {
        self.state.remove(fact)
    }

    pub fn has_key(&self, fact: &F) -> bool {
        self.state.contains_key(fact)
    }

    pub fn clear(&mut self) {
        self.state.clear();
    }
    pub fn iter(&self) -> Iter<F, V> {
        self.state.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<F, V> {
        self.state.iter_mut()
    }
}

impl<F, V> Default for WorldState<F, V>
where
    F: WorldStateType + Hash,
    V: WorldStateType,
{
    fn default() -> Self {
        Self {
            state: HashMap::default(),
        }
    }
}

impl<F, V> Add for WorldState<F, V>
where
    F: WorldStateType + Hash,
    V: WorldStateType,
{
    type Output = Self;

    fn add(self, other: WorldState<F, V>) -> WorldState<F, V> {
        let mut result = self;
        for (fact, value) in other.state.iter() {
            result.state.set(fact.clone(), value.clone());
        }
        result
    }
}
