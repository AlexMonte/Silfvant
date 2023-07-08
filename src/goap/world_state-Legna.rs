use bevy::{
    prelude::{FromReflect, Reflect},
    reflect::ReflectRef,
};
use std::hash::Hash;
use std::ops::Add;

use bevy::utils::hashbrown::hash_map::*;

use crate::utility::{QIVec3, QVec3};

#[derive(Eq, Hash, PartialEq, Clone, Reflect)]
pub enum WorldFact {
    None,
    Safe,
    CollectResource,
    IsAtPosition,
    IsEnemy,
}
#[derive(Eq, Hash, PartialEq, Clone, Reflect)]
pub enum WorldValue {
    None,
    Is(bool),
    Entity(u32),
    Position(QIVec3),
    PrecisePosition(QVec3),
    State(StateValue),
}

#[derive(Eq, Hash, PartialEq, Clone, Reflect)]
pub enum StateValue {
    Dead,
    Alive,
}

impl FromReflect for StateValue {
    fn from_reflect(value: &dyn Reflect) -> Option<Self> {
        if let ReflectRef::Enum(e) = value.reflect_ref() {
            match e.name() {
                "Dead" => Some(StateValue::Dead),
                "Alive" => Some(StateValue::Alive),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct WorldState<F: Eq + PartialEq + Hash + Clone, V: Eq + PartialEq + Clone>(HashMap<F, V>);

impl<F, V> WorldState<F, V>
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
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
impl<F, V> Hash for WorldState<F, V>
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (key, value) in self.iter() {
            key.hash(state);
            value.hash(state);
        }
    }
}

unsafe impl<F, V> Sync for WorldState<F, V>
where
    HashMap<F, V>: Sync,
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
}

unsafe impl<F, V> Send for WorldState<F, V>
where
    HashMap<F, V>: Send,
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
}
// #[derive(Resource)]
// pub struct WorldState(HashMap<Entity, EntityState>);

// impl WorldState {
//     pub fn new() -> Self {
//         WorldState(HashMap::new())
//     }

//     pub fn add_fact(&mut self, entity: Entity, name: String, fact: WorldFact) {
//         let entity_state = self.0.entry(entity).or_insert_with(EntityState::new);
//         entity_state.facts.insert(name, fact);
//     }

//     pub fn get_fact(&self, entity: Entity, name: &str) -> Option<&WorldFact> {
//         self.0
//             .get(&entity)
//             .and_then(|entity_state| entity_state.facts.get(name))
//     }

//     pub fn remove_fact(&mut self, entity: Entity, name: &str) -> Option<WorldFact> {
//         self.0
//             .get_mut(&entity)
//             .and_then(|entity_state| entity_state.facts.remove(name))
//     }

//     pub fn remove_entity(&mut self, entity: Entity) -> Option<EntityState> {
//         self.0.remove(&entity)
//     }

//     pub fn get_entity_state(&self, entity: Entity) -> Option<&EntityState> {
//         self.0.get(&entity)
//     }
// }
// //#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
// //pub enum ResourceType {
// //  Food,
// //    Water,
// // Add other resource types as needed
// //}
// #[derive(Component, Clone, Debug)]
// pub struct EntityState {
//     pub facts: HashMap<String, WorldFact>,
// }
// impl EntityState {
//     pub fn new() -> Self {
//         Self {
//             facts: HashMap::new(),
//         }
//     }

//     pub fn add_fact(&mut self, name: String, fact: WorldFact) {
//         self.facts.insert(name, fact);
//     }

//     pub fn get_fact(&self, name: &str) -> Option<&WorldFact> {
//         self.facts.get(name)
//     }

//     pub fn remove_fact(&mut self, name: &str) -> Option<WorldFact> {
//         self.facts.remove(name)
//     }

//     pub fn remove_entity(&mut self) -> Option<EntityState> {
//         let mut entity_state = EntityState::new();
//         std::mem::swap(&mut entity_state, self);
//         Some(entity_state)
//     }
// }

// impl PartialEq for WorldState {
//     fn eq(&self, other: &Self) -> bool {
//         let facts_self = self.clone();
//         let facts_other = other.clone();

//         facts_self == facts_other
//     }
// }

// impl Eq for WorldState {}

// #[derive(Clone, Debug, PartialOrd, Ord)]
// pub enum WorldFact {
//     Position(QIVec3), // Relative Position
//     PrecisePosition(QVec3),
//     Rotation(i64), // Absolute Position
//     //  Interacting(i32),    // Used in squads so that squad members don't interact with the same entity
//     IsSafe(bool), // No enemies nearby (bool) false = in danger
//                   // HasResource(ResourceType), // Resource Type (enum) Food, Water, etc.
//                   // InRange(QIVec3),           // Range
//                   // HasEnergy(i32),   // Energy Level                   // Add other world facts as needed
// }

// impl PartialEq for WorldFact {
//     fn eq(&self, other: &Self) -> bool {
//         use WorldFact::*;
//         match (self, other) {
//             (Position(a), Position(b)) => a == b,
//             (PrecisePosition(a), PrecisePosition(b)) => a == b,
//             (Rotation(a), Rotation(b)) => a == b,
//             // (Interacting(a), Interacting(b)) => a == b,
//             //       (IsSafe(a), IsSafe(b)) => a == b,
//             //       (HasResource(a), HasResource(b)) => a == b,
//             //         (InRange(a), InRange(b)) => a == b,
//             //          (HasEnergy(a), HasEnergy(b)) => a == b,
//             _ => false,
//         }
//     }
// }

//impl Eq for WorldFact {}
