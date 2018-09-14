use std::borrow::Cow;

use cgmath::Vector3;

use crate::graph::{Constant, InputState, PortId};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, PartialEq, Eq)]
pub enum DataType {
    Scalar,
    Vector,
    Mesh,
    Texture,
    Command,
}

impl DataType {
    pub fn default_constant(&self) -> Option<Constant> {
        match self {
            DataType::Scalar => Some(Constant::Scalar(0.0)),
            DataType::Vector => Some(Constant::Vector(Vector3::new(0.0, 0.0, 0.0))),
            _ => None,
        }
    }

    pub fn can_connect_to(&self, other: DataType) -> bool {
        match (self, other) {
            (DataType::Mesh, DataType::Command) => true,
            (DataType::Texture, DataType::Command) => true,
            _ => *self == other,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InputMetadata {
    pub name: &'static str,
    pub required: bool,
    pub multiple: bool,
    pub default: Option<Constant>,
    pub data_type: DataType,
}

impl InputMetadata {
    /// Return the default constant value for this input
    pub fn default_constant(&self) -> Option<Constant> {
        self.default.or_else(|| self.data_type.default_constant())
    }

    pub fn default_state(&self) -> InputState {
        self.default_constant()
            .map(|constant| InputState::Constant(constant))
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
pub struct OutputMetadata {
    pub name: &'static str,
    pub data_type: DataType,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub name: &'static str,
    pub description: Option<&'static str>,
    pub inputs: Vec<InputMetadata>,
    pub outputs: Vec<OutputMetadata>,
}

impl Metadata {
    #[inline]
    pub fn get_input(&self, id: impl Into<PortId>) -> Option<&InputMetadata> {
        let id = id.into();
        self.inputs.get(id.0)
    }

    #[inline]
    pub fn iter_inputs(&self) -> impl Iterator<Item = (PortId, &InputMetadata)> {
        self.inputs
            .iter()
            .enumerate()
            .map(|(i, meta)| (i.into(), meta))
    }

    #[inline]
    pub fn get_output(&self, id: impl Into<PortId>) -> Option<&OutputMetadata> {
        let id = id.into();
        self.outputs.get(id.0)
    }

    #[inline]
    pub fn iter_outputs(&self) -> impl Iterator<Item = (PortId, &OutputMetadata)> {
        self.outputs
            .iter()
            .enumerate()
            .map(|(i, meta)| (i.into(), meta))
    }
}
