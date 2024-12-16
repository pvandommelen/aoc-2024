use std::collections::BinaryHeap;
use std::ops::ControlFlow;

/// Push/pop stack implementation with an optimization for the last entry.
pub struct StateStack<S> {
    next: Option<S>,
    states: Vec<S>,
}
impl<S> StateStack<S> {
    pub fn push(&mut self, state: S) {
        if self.next.is_none() {
            self.next = Some(state);
        } else {
            self.states.push(state);
        }
    }
    fn pop(&mut self) -> Option<S> {
        self.next.take().or_else(|| self.states.pop())
    }
}

impl<S> Extend<S> for StateStack<S> {
    fn extend<T: IntoIterator<Item = S>>(&mut self, iter: T) {
        iter.into_iter().for_each(|s| self.push(s));
    }
}

pub fn solve_depth_first<F, S>(mut next: F, states: Vec<S>)
where
    F: FnMut(&mut StateStack<S>, S),
{
    let mut stack = StateStack { next: None, states };

    while let Some(current) = stack.pop() {
        next(&mut stack, current);
    }
}

pub fn solve_priority<F, S>(mut next: F, states: Vec<S>) -> Option<S>
where
    S: Ord,
    F: FnMut(&mut BinaryHeap<S>, &S) -> ControlFlow<()>,
{
    let mut stack = states.into_iter().collect::<BinaryHeap<_>>();

    while let Some(current) = stack.pop() {
        match next(&mut stack, &current) {
            ControlFlow::Continue(_) => {}
            ControlFlow::Break(_) => return Some(current),
        }
    }
    None
}
