pub struct Spell {
    pub name: String,
    pub spell_grid: SpellGrid,
}

impl Spell {
    pub fn new(name: String, spell_grid: SpellGrid) -> Spell {
        Spell { name, spell_grid }
    }

    pub fn draw(&self) {
        spell_grid.draw();
    }
}
