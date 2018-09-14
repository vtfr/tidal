use std::marker::PhantomData;
use std::rc::Rc;

use cgmath::Vector3;
use derive_more::From;

use crate::graph::NodePortId;
use crate::interpreter::{EvaluateError, InterpreterContext, InterpreterState};
use crate::renderer::{Command, CommandList, Mesh, Texture};

pub(crate) enum Order {
    Single(Value),
    Multiple(Vec<Value>),
}

#[derive(Debug, Clone, From)]
pub(crate) enum Value {
    Scalar(f32),
    Vector3(Vector3<f32>),
    Mesh(Rc<Mesh>),
    Texture(Rc<Texture>),
    CommandList(CommandList),
}

impl TryInto<Rc<Mesh>> for Value {
    type Error = EvaluateError;

    fn try_into(self) -> Result<Rc<Mesh>, Self::Error> {
        match self {
            Value::Mesh(mesh) => Ok(mesh),
            _ => Err(EvaluateError::GenericError),
        }
    }
}

impl TryInto<CommandList> for Value {
    type Error = EvaluateError;

    fn try_into(self) -> Result<CommandList, Self::Error> {
        match self {
            Value::Mesh(mesh) => Ok(CommandList::from(Command::AddObject(mesh))),
            Value::CommandList(command_list) => Ok(command_list),
            _ => Err(EvaluateError::GenericError),
        }
    }
}

impl TryInto<f32> for Value {
    type Error = EvaluateError;

    fn try_into(self) -> Result<f32, Self::Error> {
        match self {
            Value::Scalar(c) => Ok(c),
            _ => Err(EvaluateError::GenericError),
        }
    }
}

impl TryInto<Vector3<f32>> for Value {
    type Error = EvaluateError;

    fn try_into(self) -> Result<Vector3<f32>, Self::Error> {
        match self {
            Value::Vector3(v) => Ok(v),
            _ => Err(EvaluateError::GenericError),
        }
    }
}

#[derive(Debug)]
pub struct MultipleValue {
    pub(crate) values: Vec<Value>,
}

impl<T> TryInto<Multiple<T>> for MultipleValue {
    type Error = EvaluateError;

    fn try_into(self) -> Result<Multiple<T>, Self::Error> {
        Ok(Multiple {
            values: self.values,
            marker: Default::default(),
        })
    }
}

pub struct Multiple<V> {
    pub(crate) values: Vec<Value>,
    pub(crate) marker: PhantomData<V>,
}

// impl<T> Multiple<T>
// where
//     Value: TryInto<T>,
// {
//     pub fn into_iter(self) -> impl Iterator<Item = T> {
//         self.values
//             .into_iter()
//             .flat_map(|v| -> Option<T> { v.try_into().ok() })
//     }
// }
