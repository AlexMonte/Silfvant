use crate::goap::memory::Memory;
use bevy::prelude::*;
use std::hash::Hash;

use super::world_state::*;

pub trait Senses<F, V>
where
    F: WorldStateType + Hash,
    V: WorldStateType,
{
    fn init(&self, men: &mut Memory<F, V>);

    fn update(&self, mem: &mut Memory<F, V>);
}

pub fn initialized_senses<Sense: Senses<WorldFact, WorldValue> + Component>(
    mut query: Query<(&Sense, &mut Memory<WorldFact, WorldValue>)>,
) {
    for (&senses, mut memory) in query.iter_mut() {
        senses.init(&mut memory);
    }
}
