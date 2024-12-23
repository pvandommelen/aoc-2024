use rustc_hash::FxHashSet;
use std::collections::BinaryHeap;
use std::hash::Hash;
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

pub fn solve_breadth_first_dedup<F, S>(
    mut next: F,
    states: impl IntoIterator<Item = S>,
) -> Option<(S, usize)>
where
    F: FnMut(&mut FxHashSet<S>, &S, usize) -> ControlFlow<()>,
    S: Hash + Eq,
{
    let mut round = 0;
    let mut states: FxHashSet<_> = states.into_iter().collect();
    let mut next_states = FxHashSet::default();

    loop {
        for state in states.drain() {
            match next(&mut next_states, &state, round) {
                ControlFlow::Break(_) => return Some((state, round)),
                ControlFlow::Continue(_) => {}
            };
        }
        if next_states.is_empty() {
            return None;
        }
        std::mem::swap(&mut states, &mut next_states);
        round += 1;
    }
}

pub fn solve_breadth_first<F, S>(
    mut next: F,
    states: impl IntoIterator<Item = S>,
) -> Option<(S, usize)>
where
    F: FnMut(&mut Vec<S>, &S, usize) -> ControlFlow<()>,
{
    let mut round = 0;
    let mut states: Vec<_> = states.into_iter().collect();
    let mut next_states = Vec::default();

    loop {
        for state in states.drain(..) {
            match next(&mut next_states, &state, round) {
                ControlFlow::Break(_) => return Some((state, round)),
                ControlFlow::Continue(_) => {}
            };
        }
        if next_states.is_empty() {
            return None;
        }
        std::mem::swap(&mut states, &mut next_states);
        round += 1;
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

pub trait Stack<S> {
    fn push(&mut self, item: S);
}

pub struct PriorityDedupStack<S: Ord> {
    stack: BinaryHeap<S>,
    stack_set: FxHashSet<S>,
}
impl<S: Ord> Default for PriorityDedupStack<S> {
    fn default() -> Self {
        PriorityDedupStack {
            stack: Default::default(),
            stack_set: Default::default(),
        }
    }
}
impl<S: Hash + Ord + Clone> Stack<S> for PriorityDedupStack<S> {
    fn push(&mut self, item: S) {
        if !self.stack_set.contains(&item) {
            self.stack_set.insert(item.clone());
            self.stack.push(item);
        }
    }
}
impl<S: Ord> PriorityDedupStack<S> {
    fn pop(&mut self) -> Option<S>
    where
        S: Hash,
    {
        let item = self.stack.pop()?;
        self.stack_set.remove(&item);
        Some(item)
    }
}

pub fn solve_priority_dedup<F, S>(mut next: F, states: Vec<S>) -> Option<S>
where
    S: Ord + Hash + Clone,
    F: FnMut(&mut PriorityDedupStack<S>, &S) -> ControlFlow<()>,
{
    let mut stack = PriorityDedupStack::default();
    states.into_iter().for_each(|state| {
        stack.push(state);
    });

    while let Some(current) = stack.pop() {
        match next(&mut stack, &current) {
            ControlFlow::Continue(_) => {}
            ControlFlow::Break(_) => return Some(current),
        }
    }
    None
}
