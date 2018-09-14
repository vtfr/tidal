use std::collections::HashSet;
use std::ops::Deref;

use derive_more::Constructor;
use eframe::egui::Vec2;

use tidal_core::cgmath::Vector2;
use tidal_core::graph::{Constant, Graph, InputState, Node, NodeId, NodePortId, Placement, PortId};
use tidal_core::operator::Operator;

use crate::state::store::Command;
use crate::state::State;

#[derive(Debug, Clone)]
pub enum GraphCommand {
    CreateNode {
        operator: Operator,
        position: Vector2<f32>,
    },
    MoveNode {
        node_id: NodeId,
        delta: Vec2,
    },
    MoveNodes {
        node_ids: HashSet<NodeId>,
        delta: Vec2,
    },
    ConnectNode {
        output: NodePortId,
        input: NodePortId,
        placement: Placement,
    },
    Disconnect {
        input: NodePortId,
        index: usize,
    },
    ChangeConstant {
        node_id: NodeId,
        port_id: PortId,
        constant: Constant,
    },
}

impl GraphCommand {
    pub fn apply(&self, state: &mut Graph) {
        match self {
            GraphCommand::CreateNode { operator, position } => {
                let node = Node::new(operator.clone(), *position);

                state.nodes.push(node);
            }
            GraphCommand::MoveNode { node_id, delta } => {
                if let Some(node) = state.get_node_mut(*node_id) {
                    node.position += Vector2::new(delta.x, delta.y);
                }
            }
            GraphCommand::MoveNodes { node_ids, delta } => {
                for node_id in node_ids {
                    if let Some(node) = state.get_node_mut(*node_id) {
                        node.position += Vector2::new(delta.x, delta.y);
                    }
                }
            }
            GraphCommand::ConnectNode {
                output,
                input,
                placement,
            } => {
                state.connect(output, input, *placement);
            }
            GraphCommand::Disconnect { input, index } => state.disconnect(input, *index),
            GraphCommand::ChangeConstant {
                node_id,
                port_id,
                constant,
            } => {
                if let Some(node) = state.get_node_mut(*node_id) {
                    if let Some(port) = node.get_input_state_mut(*port_id) {
                        *port = InputState::Constant(*constant)
                    }
                }
            }
        }
    }

    pub fn can_merge_with(&self, other: &GraphCommand) -> bool {
        type C = GraphCommand;

        match (self, other) {
            (
                C::MoveNode { node_id, .. },
                C::MoveNode {
                    node_id: other_node_id,
                    ..
                },
            ) => *node_id == *other_node_id,
            (
                C::MoveNodes { node_ids, .. },
                C::MoveNodes {
                    node_ids: other_node_ids,
                    ..
                },
            ) => *other_node_ids == *node_ids,
            (
                C::ChangeConstant {
                    node_id, port_id, ..
                },
                C::ChangeConstant {
                    node_id: other_node_id,
                    port_id: other_port_id,
                    ..
                },
            ) => *other_node_id == *node_id && *other_port_id == *port_id,
            _ => false,
        }
    }
}
