use std::any::Any;

use crate::engine::di::Container;
use crate::engine::engine::Engine;
use crate::operator::operator::{MaterializedOperatorImpl, Operator};
use crate::operator::registry::OperatorRegistry;

#[derive(Default)]
pub struct App {
    pub container: Container,
}

impl App {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_operator<O>(mut self) -> App
    where
        O: Operator + 'static,
    {
        self.container
            .get_mut_or_default::<OperatorRegistry>()
            .register::<O>();
        self
    }

    pub fn add_resource<R>(mut self, resource: R) -> App
    where
        R: Any + 'static,
    {
        self.container.register(resource);
        self
    }

    pub async fn run(self) {
        let engine = Engine::new(self.container);

        engine.run().await
    }
}
