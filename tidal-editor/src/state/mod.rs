use tidal_core::graph::Graph;

pub mod graph;
pub mod store;

/// State represents the entire state of the application. Doesn't include
/// derivative states such as widget states.
#[derive(Debug, Clone)]
pub struct State {
    pub graph: Graph,
}
