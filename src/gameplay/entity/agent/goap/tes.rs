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
pub struct WorldState(HashMap<String, WorldFact>);

impl WorldState {
    pub fn new(key: String, value: WorldFact) -> Self {
        let mut world_state = HashMap::new();
        Self(world_state)
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn has_conflicting_facts(&self, other: WorldState) -> bool {
        for (fact, value) in self.0.iter() {
            if let Some(other_value) = other.try_to_get(fact) {
                if value != other_value {
                    log!("Conflict: {0} with {1} and {2}", fact, value, other_value);
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
        predicate: Option<Box<dyn Fn(String, WorldFact) -> bool>>,
    ) -> i32 {
        let mut count = 0;
        let buffer = self.clone();

        self.clear();

        use std::collections::HashMap;

        use crate::core::game::{Game, GameResult};
        use crate::core::world::WorldState;
        use crate::core::world::{WorldFact, WorldValue};

        pub struct WorldStateTracker {
            pub states: Vec<WorldState<WorldFact, WorldValue>>,
            pub conflicts: Vec<(
                WorldState<WorldFact, WorldValue>,
                WorldState<WorldFact, WorldValue>,
            )>,
        }

        impl WorldStateTracker {
            pub fn new() -> Self {
                Self {
                    states: vec![],
                    conflicts: vec![],
                }
            }

            pub fn add(&mut self, state: WorldState<WorldFact, WorldValue>) {
                self.states.push(state);
            }

            pub fn update(&mut self, game: &Game) -> GameResult<()> {
                let mut buffer = WorldState::new(WorldFact::None, WorldValue::None);
                let mut state = WorldState::new(WorldFact::None, WorldValue::None);

                for entity in game.entities.iter() {
                    let is_safe = game
                        .world
                        .query_one::<&bool>(entity)
                        .map_or(false, |is_safe| *is_safe);
                    let is_enemy = game
                        .world
                        .query_one::<&bool>(entity)
                        .map_or(false, |is_enemy| *is_enemy);
                    let has_resource = game
                        .world
                        .query_one::<&bool>(entity)
                        .map_or(false, |has_resource| *has_resource);

                    if let Some(entity_id) = game.world.get_entity(entity) {
                        buffer.set(WorldFact::None, WorldValue::Entity(entity_id));
                    }
                    if is_safe {
                        buffer.set(WorldFact::Safe, WorldValue::Is(is_safe));
                    }
                    if is_enemy {
                        buffer.set(WorldFact::Enemy, WorldValue::Is(is_enemy));
                    }
                    if has_resource {
                        buffer.set(WorldFact::CollectResource, WorldValue::Is(has_resource));
                    }

                    state.add_from_state(&buffer);
                }

                if !self.states.is_empty()
                    && state.has_conflicting_facts(self.states.last().unwrap().clone())
                {
                    self.conflicts
                        .push((state.clone(), self.states.last().unwrap().clone()));
                }
                self.states.push(state);
                Ok(());

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
                pub struct WorldState<String: Eq + PartialEq + Clone, WorldFact: Eq + PartialEq + Clone>(
                    HashMap,
                );

                impl WorldState
                where
                    String: Eq + PartialEq + Hash + Clone,
                    WorldFact: Eq + PartialEq + Clone,
                    HashMap: Hash,
                {
                    pub fn new(key: String, value: WorldFact) -> Self {
                        let mut world_state = HashMap::new();
                        world_state.insert(key, value);
                        Self(world_state)
                    }

                    pub fn count(&self) -> usize {
                        self.0.len()
                    }

                    pub fn has_conflicting_facts(&self, other: WorldState) -> bool {
                        for (fact, value) in self.0.iter() {
                            if let Some(other_value) = other.try_to_get(fact) {
                                if value != other_value {
                                    log!(
                                        "Conflict: {0} with {1} and {2}",
                                        fact,
                                        value,
                                        other_value
                                    );
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
                        predicate: Option<Box<dyn Fn(String, WorldFact) -> bool>>,
                    ) -> i32 {
                        let mut count = 0;
                        let buffer = self.clone();

                        self.clear();

                        for (fact, value) in buffer.iter() {
                            if let Some(other_value) = other.try_to_get(fact) {
                                if value != other_value && predicate.is_none()
                                    || predicate.as_ref().unwrap()(fact.clone(), value.clone())
                                {
                                    log!(
                                        "Conflict: {0} with {1} and {2}",
                                        fact,
                                        value,
                                        other_value
                                    );
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

                    pub fn get(&self, fact: &String) -> &WorldFact {
                        self.0.get(fact).unwrap()
                    }
                    pub fn get_mut(&mut self, fact: &String) -> &mut WorldFact {
                        self.0.get_mut(fact).unwrap()
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

                impl Default for WorldState
                where
                    String: Eq + PartialEq + Hash + Clone,
                    WorldFact: Eq + PartialEq + Clone,
                {
                    fn default() -> Self {
                        Self {
                            0: HashMap::default(),
                        }
                    }
                }

                impl Add for WorldState
                where
                    String: Eq + PartialEq + Hash + Clone,
                    WorldFact: Eq + PartialEq + Clone,
                {
                    type Output = Self;

                    fn add(self, other: WorldState) -> WorldState {
                        let mut result = self;
                        for (fact, value) in other.iter() {
                            result.set(fact.clone(), value.clone());
                        }
                        Self(result.0)
                    }
                }

                for (fact, value) in buffer.iter() {
                    if let Some(other_value) = other.try_to_get(fact) {
                        if value != other_value && predicate.is_none()
                            || predicate.as_ref().unwrap()(fact.clone(), value.clone())
                        {
                            log!("Conflict: {0} with {1} and {2}", fact, value, other_value);
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

            pub fn get(&self, fact: &String) -> &WorldFact {
                self.0.get(fact).unwrap()
            }
            pub fn get_mut(&mut self, fact: &String) -> &mut WorldFact {
                self.0.get_mut(fact).unwrap()
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

        impl Default for WorldState
        where
            String: Eq + PartialEq + Hash + Clone,
            WorldFact: Eq + PartialEq + Clone,
        {
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
    }
}