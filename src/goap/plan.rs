use crate::goap::action::ActionState;
use std::collections::vec_deque::{Iter, IterMut, VecDeque};
use std::hash::Hash;

pub struct Plan<F: Eq + PartialEq + Hash + Clone, V: Eq + PartialEq + Clone>(
    VecDeque<ActionState<F, V>>,
);

impl<F, V> Plan<F, V>
where
    F: Eq + PartialEq + Hash + Clone,
    V: Eq + PartialEq + Clone,
{
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(&mut self, action_state: ActionState<F, V>) {
        self.0.push_back(action_state);
    }

    pub fn pop(&mut self) -> Option<ActionState<F, V>> {
        self.0.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> Iter<'_, ActionState<F, V>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, ActionState<F, V>> {
        self.0.iter_mut()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }
}
