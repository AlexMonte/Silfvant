use crate::goap::memory::Memory;
use bevy::prelude::*;
use std::hash::Hash;

use super::world_state::{WorldFact, WorldValue};

pub trait Senses<F, V>
where
    F: Eq + PartialEq + Hash + Clone + Reflect,
    V: Eq + PartialEq + Clone + Reflect,
{
    fn init(&self, men: &mut Memory<F, V>);

    fn update(&self, mem: &mut Memory<F, V>);
}

pub fn initialized_senses<S: Senses<WorldFact, WorldValue> + Component>(
    mut query: Query<(&S, &mut Memory<WorldFact, WorldValue>)>,
) {
    for (&senses, mut memory) in query.iter_mut() {
        senses.init(&mut memory);
    }
}
