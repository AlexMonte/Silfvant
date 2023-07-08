use bevy::prelude::*;


#[derive(Resourse)] 
pub struct SpellCache {
    pub spells: Hashmap<Spell>,
}