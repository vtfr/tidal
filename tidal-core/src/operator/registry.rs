use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Write};

use crate::operator::operator::{
    MaterializedOperator, MaterializedOperatorImpl, Metadata, Operator, OperatorId,
};

pub struct OperatorRegistryNode {
    create_fn: Box<dyn Fn() -> Box<dyn MaterializedOperator>>,
    metadata: &'static Metadata,
    operator_id: OperatorId,
}

impl Debug for OperatorRegistryNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("OperatorNode({:?})", self.operator_id))
    }
}

impl OperatorRegistryNode {
    pub fn new<O>() -> Self
    where
        O: Operator + 'static,
    {
        let create_fn = move || {
            Box::new(MaterializedOperatorImpl::new(O::new())) as Box<dyn MaterializedOperator>
        };

        Self {
            create_fn: Box::new(create_fn),
            operator_id: OperatorId::of::<O>(),
            metadata: O::metadata(),
        }
    }

    #[inline]
    pub fn operator_id(&self) -> OperatorId {
        self.operator_id
    }

    /// Returns this operator's metadata
    #[inline]
    pub fn metadata(&self) -> &'static Metadata {
        self.metadata
    }

    /// Creates a new instance of this operator
    #[inline]
    pub fn create(&self) -> Box<dyn MaterializedOperator> {
        (self.create_fn)()
    }
}

#[derive(Default, Debug)]
pub struct OperatorRegistry {
    registry: HashMap<OperatorId, OperatorRegistryNode>,
}

impl OperatorRegistry {
    /// Registers a new operator.
    pub fn register<O: Operator + 'static>(&mut self) {
        let node = OperatorRegistryNode::new::<O>();

        self.registry.insert(node.operator_id(), node);
    }

    /// Instantiates a [`MaterializedOperator`] if it is registered.
    pub fn instantiate(&self, id: OperatorId) -> Option<Box<dyn MaterializedOperator>> {
        self.registry.get(&id).map(|node| node.create())
    }
}
