use std::cell::UnsafeCell;
use std::marker::PhantomData;

use crate::engine::di::Container;
use crate::graph::graph::Graph;
use crate::graph::runner::GraphRunner;

#[derive(Debug)]
pub struct Engine {
    container: Container,
}

impl Engine {
    #[inline]
    pub fn new(container: Container) -> Self {
        Self { container }
    }

    /// Runs the engine.
    pub async fn run(&self) {
        let runner = self
            .container
            .get_cloned::<GraphRunner>()
            .expect("no runner");

        runner.run(&self.container)
    }
}
