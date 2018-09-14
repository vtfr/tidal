use serde::{Deserialize, Serialize};

use crate::graph::Metadata;
use crate::interpreter::Evaluate;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Operator(pub String);

/// Registry node for Metadata
pub struct OperatorMetadataRegistryNode {
    pub operator: &'static str,
    pub create_metadata: fn() -> Metadata,
}

inventory::collect!(OperatorMetadataRegistryNode);

/// Registry node for Evaluators
pub struct EvaluatorRegistryNode {
    pub operator: &'static str,
    pub create_operator: fn() -> Box<dyn Evaluate>,
}

inventory::collect!(EvaluatorRegistryNode);

impl Operator {
    #[inline]
    pub fn scene() -> Operator {
        Operator("Scene".into())
    }

    pub fn describe(&self) -> Metadata {
        let node: &OperatorMetadataRegistryNode = inventory::iter::<OperatorMetadataRegistryNode>()
            .find(|node| self.0 == node.operator)
            .unwrap();

        (node.create_metadata)()
    }

    pub fn to_evaluator(&self) -> Box<dyn Evaluate> {
        let node = inventory::iter::<EvaluatorRegistryNode>()
            .find(|node| self.0 == node.operator)
            .unwrap();

        (node.create_operator)()
    }

    pub fn all() -> Vec<Operator> {
        inventory::iter::<OperatorMetadataRegistryNode>()
            .map(|node| Operator(node.operator.into()))
            .collect()
    }
}
