use serde::{Deserialize, Serialize};

use crate::graph::Graph;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Demo {
    pub graph: Graph,
}
