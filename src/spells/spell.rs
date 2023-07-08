pub struct SpellData {}

pub trait Spell {
    pub fn execute(&self, entity: Entity, context: SpellContext) -> Result<(), SpellError>;
}
