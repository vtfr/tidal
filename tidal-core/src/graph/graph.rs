use std::slice::Iter;

use crate::graph::node::{Node, NodeId, Nodes};

#[derive(Debug, Default)]
pub struct Graph {
    nodes: Nodes,
}

/// Determines how a value should be stored.
///
/// For inputs that accept multiple values:
/// - [`ValuePlacement::At`] inserts the [`Value`] at the provided index, replacing the
///   previous value;
/// - [`ValuePlacement::After`] insert the [`Value`] after the provided index,
///   moving all subsequent values to the right;
///
/// For inputs that accept a single value, both variants will replace the index zero. For
/// simplicity, the method [single] is available for constructing single value input placements.
///
/// [single]: ValuePlacement::single
#[derive(Debug)]
pub enum ValuePlacement {
    At(usize),
    After(usize),
}

impl ValuePlacement {
    /// Inserts the value at the default zero index.
    #[inline]
    pub const fn single() -> Self {
        Self::At(0)
    }
}

// pub struct InputPort(pub usize, usize);
// pub struct OutputPort(pub usize);
//
// pub struct InputId(NodeId, InputPort);
// pub struct OutputId(OutputId, OutputPort);

impl Graph {
    #[inline]
    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    #[inline]
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }
}
//     pub fn assign_constant<T>(
//         &mut self,
//         id: NodeId,
//         input: InputId,
//         placement: ValuePlacement,
//         value: T,
//     ) {
//         todo!()
//     }
//
//     pub fn assign_boxed_constant(
//         &mut self,
//         id: NodeId,
//         input: InputId,
//         placement: ValuePlacement,
//         value: Box<dyn Value>,
//     ) {
//     }
//
//     /// Stores an output value into a [`Node`]s output.
//     pub fn store_output<T>(&mut self, id: NodeId, input: InputId, value: T) {
//         todo!()
//     }
//
//     /// Retrieves a typed output from a [`Node`].
//     pub fn get_node_output<T>(&mut self, id: NodeId, output: OutputId) -> Option<T> {
//         let node = self.nodes.get(id)?;
//
//         let x = node.inputs.get(0)?;
//     }
//
//     pub fn connect(&mut self, id: NodeId, output: OutputId) -> Option<T> {
//         let node = self.nodes.get(id)?;
//
//         let x = node.inputs.get(0)?;
//     }
//
//     /// Retrieves a boxed output from a [`Node`].
//     pub fn get_node_boxed_output(
//         &mut self,
//         id: NodeId,
//         output: OutputId,
//     ) -> Option<Box<dyn Value>> {
//         let node = self.nodes.get(id)?;
//
//         let x = node.inputs.get(0)?;
//     }
// }
//
// pub struct Connection {}
//
// /// An [`Iterator`] over all the [connections] between node ports.
// ///
// /// [connections]: Connection
// pub struct ConnectionIter<'a> {
//     nodes_iter: NodeIter<'a>,
//     current_input_iter: Option<Iter<'a, BoxedInputSlot>>,
// }
//
// impl<'a> Iterator for ConnectionIter<'a> {
//     type Item = Connection;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }
