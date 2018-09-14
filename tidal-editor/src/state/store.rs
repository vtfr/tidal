use std::any::Any;
use std::cell::RefCell;
use std::time::{Duration, Instant};

use delegate::delegate;
use derive_more::From;

use crate::state::graph::GraphCommand;
use crate::state::State;

const MAXIMUM_DURATION: Duration = Duration::from_millis(100);

#[derive(Debug, Clone, From)]
pub enum Command {
    Graph(GraphCommand),
}

impl Command {
    pub fn apply(&self, state: &mut State) {
        match self {
            Command::Graph(c) => c.apply(&mut state.graph),
        }
    }

    pub fn can_merge_with(&self, other: &Command) -> bool {
        match (self, other) {
            (Command::Graph(a), Command::Graph(b)) => a.can_merge_with(b),
            // _ -> false,
        }
    }
}

pub trait Dispatcher {
    fn dispatch(&self, c: Command);
}

struct LastCommand {
    command: Command,
    when: Instant,
}

pub struct Store {
    state: State,
    queue: RefCell<Vec<Command>>,
    undo: Vec<State>,
    redo: Vec<State>,
    last_command: Option<LastCommand>,
}

impl Store {
    pub fn new(state: State) -> Self {
        Self {
            state,
            queue: Default::default(),
            undo: Default::default(),
            redo: Default::default(),
            last_command: None,
        }
    }

    pub fn undo(&mut self) {
        if let Some(state) = self.undo.pop() {
            self.redo.push(self.state.clone());
            self.state = state;
        }
    }

    pub fn redo(&mut self) {
        if let Some(state) = self.redo.pop() {
            self.undo.push(self.state.clone());
            self.state = state;
        }
    }

    pub fn run(&mut self) {
        let mut queue = self.queue.replace(vec![]);

        for command in queue.into_iter().rev() {
            self.apply(command)
        }
    }

    pub fn dispatch(&self, c: impl Into<Command>) {
        self.queue.borrow_mut().push(c.into())
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    fn apply(&mut self, command: Command) {
        let now = Instant::now();

        if let Some(last) = &self.last_command {
            if last.command.can_merge_with(&command) {
                if now.duration_since(last.when) < MAXIMUM_DURATION {
                    // Apply directly.
                    //
                    // No need to store it in the undo stack because
                    // it will already contain the previous valid state.
                    command.apply(&mut self.state);

                    self.last_command = Some(LastCommand { command, when: now });
                    return;
                }
            }
        }

        self.undo.push(self.state.clone());

        command.apply(&mut self.state);

        self.last_command = Some(LastCommand { command, when: now });
    }
}
