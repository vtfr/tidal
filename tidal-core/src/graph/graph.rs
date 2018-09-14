use cgmath::{Vector2, Zero};
use serde::{Deserialize, Serialize};

use crate::graph::{Node, NodeId, NodePortId, Placement};
use crate::operator::Operator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: Vec<Node>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            nodes: vec![Node::new(Operator::scene(), Vector2::zero())],
        }
    }
}

impl Graph {
    #[inline]
    pub fn get_node(&self, id: impl Into<NodeId>) -> Option<&Node> {
        let id = id.into();
        self.nodes.get(id.0)
    }

    #[inline]
    pub fn iter_nodes(&self) -> impl Iterator<Item = (NodeId, &Node)> {
        self.nodes
            .iter()
            .enumerate()
            .map(|(id, node)| (id.into(), node))
    }

    #[inline]
    pub fn get_node_mut(&mut self, id: impl Into<NodeId>) -> Option<&mut Node> {
        let id = id.into();
        self.nodes.get_mut(id.0)
    }

    #[inline]
    pub fn can_connect(&self, output: &NodePortId, input: &NodePortId) -> bool {
        self.can_connect_impl(output, input).is_some_and(|v| v)
    }

    fn can_connect_impl(&self, output: &NodePortId, input: &NodePortId) -> Option<bool> {
        let output_node = self.get_node(output.get_node_id())?;
        let output_metadata = output_node.operator.describe();
        let output_port = output_metadata.get_output(output.get_port_id())?;

        let input_node = self.get_node(input.get_node_id())?;
        let input_metadata = input_node.operator.describe();
        let input_port = input_metadata.get_input(input.get_port_id())?;

        if !output_port.data_type.can_connect_to(input_port.data_type) {
            return Some(false);
        }

        Some(true)
    }

    pub fn connect(&mut self, output: &NodePortId, input: &NodePortId, strategy: Placement) {
        if self.can_connect(output, input) {
            let input_node = self.get_node_mut(input.get_node_id()).unwrap();

            input_node.connect(input.get_port_id(), *output, strategy);
        }
    }

    pub fn disconnect(&mut self, input: &NodePortId, index: usize) {
        if let Some(input_node) = self.get_node_mut(input.get_node_id()) {
            input_node.disconnect(input.get_port_id(), index);
        };
    }

    #[inline]
    pub fn can_remove_node(&self, node_id: &NodeId) -> bool {
        return !node_id.is_root();
    }

    pub fn create_node(&mut self, operator: Operator, position: Vector2<f32>) {
        self.nodes.push(Node::new(operator, position))
    }

    pub fn remove_node(&mut self, node_id: &NodeId) {
        if !self.can_remove_node(node_id) {
            return;
        }

        todo!()
    }

    // pub fn create_node(&mut self, req: &CreateNodeRequest) {
    //     let node = Node::new(req.id, req.operator.clone(), req.position);
    //
    //     // let metadata = node.operator.describe();
    //     // for input in metadata.inputs.iter() {
    //     //     let constant = input.default.or(input.data_type.default_constant());
    //     //
    //     //     if let Some(constant) = constant {
    //     //         node.set_input_constant(input.id, constant)
    //     //     } else {
    //     //         node.set_input_dangling(input.id)
    //     //     }
    //     // }
    //
    //     self.state.nodes.insert(req.id, node);
    // }
}
