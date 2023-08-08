use bevy::prelude::*;
use uuid::Uuid;

#[derive(Resource)]
pub struct SpellCache {
    pub spells: HashMap<Uuid, CompiledSpell>,
}

impl SpellCache {
    pub fn new() -> Self {
        SpellCache {
            spells: HashMap::new(),
        }
    }

    pub fn add_spell(&mut self, spell: CompiledSpell) {
        self.spells.insert(spell.id(), spell);
    }

    pub fn get_spell(&self, spell: Spell) -> Option<&CompiledSpell> {
        self.spells.get(&id)
    }
}
