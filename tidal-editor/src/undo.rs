use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::time::{Duration, Instant};

use delegate::delegate;

pub struct Undo<S> {
    last_changed: Instant,
    history: VecDeque<S>,
}

impl<S> Default for Undo<S> {
    fn default() -> Self {
        Self {
            last_changed: Instant::now(),
            history: VecDeque::new(),
        }
    }
}

impl<S> Undo<S>
where
    S: Clone + PartialEq,
{
    /// Detect if changes were made to the current state, storing the mutated state if so.
    /// Rate-limit changes so we don't spam the detector with in-between state changes.
    pub fn detect_changes(&mut self, current: &S) {
        if let Some(previous) = self.history.back() {
            if previous != current {
                if Instant::now().duration_since(self.last_changed) > Duration::from_millis(60) {
                    self.add(current);
                }
            }
        } else {
            self.add(current);
        }
    }

    /// Undoes the object
    pub fn undo(&mut self, state: &mut S) {
        if let Some(previous) = self.history.pop_back() {
            *state = previous;
        }
    }

    fn add(&mut self, current: &S) {
        self.history.push_back(current.clone());
        self.last_changed = Instant::now();
    }
}
