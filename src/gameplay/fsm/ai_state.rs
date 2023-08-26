use bevy::prelude::*;
pub enum State {
    Animate(i32),
    MoveTo(Vec3),
    Interacting(Entity),
}

pub enum Transition {
    Swap, // cancel current state and start new state
    Push, // interrupt current state and start new state
    Add,  // Add compliment animation to current state
}
#[derive(Component)]
pub struct AIState {
    pub current_state: State,
    pub transition: Option<Transition>,
}
