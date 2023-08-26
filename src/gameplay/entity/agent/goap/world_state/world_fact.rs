#[derive(Eq, Hash, PartialEq, Clone, Debug)]

/*
    @WorldFact is a fact that is stored in the world state. It is used to represent the state of the world.
*/
pub enum WorldFact {
    None,
    IsHolding,
    IsAt,
    CurrentlyNeeds,
}

impl WorldFact {
    pub fn new(value: WorldFact) -> Self {
        Self { value }
    }
}

impl Default for WorldFact {
    fn default() -> Self {
        Self {
            value: WorldFact::None,
        }
    }
}
