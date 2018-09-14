use std::borrow::Cow;
use std::vec::IntoIter;

use cgmath::{Vector2, Vector3};
use derive_more::IsVariant;
use serde::{Deserialize, Serialize};

use crate::graph::{InputMetadata, Metadata, NodePortId, PortId};
use crate::operator::Operator;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Constant {
    Scalar(f32),
    I32(i32),
    Vector(Vector3<f32>),
}

#[derive(Serialize, Deserialize, Debug, Clone, IsVariant)]
#[serde(tag = "type", content = "value")]
pub enum InputState {
    Constant(Constant),
    Connection(Vec<NodePortId>),
}

impl Default for InputState {
    fn default() -> Self {
        Self::Connection(vec![])
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node
// where
//     NodeVisual: Serialize + Deserialize<'a>,
{
    pub operator: Operator,
    pub inputs: Vec<InputState>,
    pub position: Vector2<f32>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Placement {
    /// Replaces the current connection at index
    Replace(usize),
    /// Inserts before the given index
    Insert(usize),
}

impl Node {
    pub fn new(operator: Operator, position: Vector2<f32>) -> Self {
        let metadata = operator.describe();

        let inputs = metadata
            .inputs
            .iter()
            .map(InputMetadata::default_state)
            .collect();

        Self {
            operator,
            inputs,
            position,
        }
    }

    #[inline]
    pub fn get_input_state(&self, id: impl Into<PortId>) -> Option<&InputState> {
        let id = id.into();
        self.inputs.get(id.0)
    }

    #[inline]
    pub fn get_input_state_mut(&mut self, id: impl Into<PortId>) -> Option<&mut InputState> {
        let id = id.into();
        self.inputs.get_mut(id.0)
    }

    pub fn get_described_input(
        &self,
        id: impl Into<PortId>,
    ) -> Option<(&InputState, InputMetadata)> {
        let id = id.into();
        let metadata = self.operator.describe();

        let input_metadata = metadata.get_input(id)?;
        let input = self.get_input_state(id)?;

        Some((input, input_metadata.clone()))
    }

    pub fn get_described_input_mut(
        &mut self,
        id: impl Into<PortId>,
    ) -> Option<(&mut InputState, InputMetadata)> {
        let id = id.into();
        let metadata = self.operator.describe();

        let input_metadata = metadata.get_input(id)?;
        let input = self.get_input_state_mut(id)?;

        Some((input, input_metadata.clone()))
    }

    pub fn iter_described_inputs(
        &self,
    ) -> impl Iterator<Item = (PortId, &InputState, InputMetadata)> {
        let metadata = self.operator.describe();

        self.inputs
            .iter()
            .zip(metadata.inputs.clone().into_iter())
            .enumerate()
            .map(|(i, (input, metadata))| (i.into(), input, metadata))
    }

    pub fn iter_described_inputs_mut(
        &mut self,
    ) -> impl Iterator<Item = (PortId, &mut InputState, InputMetadata)> {
        let metadata = self.operator.describe();

        self.inputs
            .iter_mut()
            .zip(metadata.inputs.clone().into_iter())
            .enumerate()
            .map(|(i, (input, metadata))| (i.into(), input, metadata))
    }

    /// Creates a new connection from this node to the source node, inserting at index
    pub fn connect(&mut self, input_id: PortId, output: NodePortId, strategy: Placement) {
        if let Some((input, meta)) = self.get_described_input_mut(input_id) {
            // Fix the strategy if not compatible with the input meta. If the input does not
            // support multiple values, then we always will use the Direct strategy.
            // let strategy = if meta.multiple {
            //     strategy
            // } else {
            //     Placement::Replace(0)
            // };

            match input {
                // If is a constant, then connect it.
                InputState::Constant(_) => {
                    *input = InputState::Connection(vec![output]);
                    return;
                }
                // Connect at location
                InputState::Connection(cs) => match strategy {
                    Placement::Replace(i) => {
                        if i < cs.len() {
                            cs[i] = output
                        }
                    }
                    Placement::Insert(i) => {
                        if i <= cs.len() {
                            cs.insert(i, output)
                        }
                    }
                },
            }
        }
    }

    /// Disconnect a node from source
    pub fn disconnect(&mut self, input_id: PortId, index: usize) {
        if let Some((input, meta)) = self.get_described_input_mut(input_id) {
            let should_reset = if let InputState::Connection(cs) = input {
                if index < cs.len() {
                    cs.remove(index);
                }

                cs.is_empty()
            } else {
                false
            };

            if should_reset {
                *input = meta.default_state()
            }
        }
    }
}
