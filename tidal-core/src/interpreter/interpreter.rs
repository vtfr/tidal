use std::cell::UnsafeCell;

use crate::demo::Demo;
use crate::graph::{Constant, Graph, InputState, NodeId, PortId};
use crate::interpreter::evaluator::{Evaluate, EvaluateError};
use crate::interpreter::value::Value;
use crate::interpreter::{EvaluateContext, MultipleValue};
use crate::renderer::Renderer;

const MAXIMUM_OUTPUTS: usize = 16;

#[derive(Clone)]
pub(crate) struct Outputs(Box<[Value; MAXIMUM_OUTPUTS]>);

impl Outputs {
    pub fn new() -> Self {
        /// SAFETY: Considering enums are stored sequentially in memory as (variant, union(values)),
        /// by creating a zeroed memory location the default variant will be the first one, which is
        /// [`Value::Scalar`], which internally is an f32. A float with all it's bits set to zero
        /// represents a 0.0f32, which is also a valid value.
        let outputs: Box<[Value; MAXIMUM_OUTPUTS]> = unsafe { Box::new_zeroed().assume_init() };

        Self(outputs)
    }

    #[inline]
    pub fn write_output(&mut self, port_id: PortId, value: Value) {
        let value_ref = self.0.get_mut(port_id.0).expect("port out of range");
        *value_ref = value
    }

    #[inline]
    pub fn get_output(&self, port_id: PortId) -> Value {
        self.0.get(port_id.0).expect("port out of range").clone()
    }
}

#[derive(Clone)]
pub(crate) struct OutputStorage(Vec<Outputs>);

impl OutputStorage {
    pub fn new(nodes_count: usize) -> Self {
        Self(vec![Outputs::new(); nodes_count])
    }

    pub fn get_output(&self, node_id: NodeId, port_id: PortId) -> Value {
        self.get_node_outputs(node_id).get_output(port_id)
    }

    pub fn write_output(&mut self, node_id: NodeId, port_id: PortId, value: Value) {
        self.get_node_outputs_mut(node_id)
            .write_output(port_id, value)
    }

    fn get_node_outputs(&self, node_id: NodeId) -> &Outputs {
        self.0.get(node_id.0).expect("node out of range")
    }

    fn get_node_outputs_mut(&mut self, node_id: NodeId) -> &mut Outputs {
        self.0.get_mut(node_id.0).expect("node out of range")
    }
}

pub(crate) struct InterpreterState {
    storage: OutputStorage,
    evaluators: Vec<Box<dyn Evaluate>>,
}

pub struct Interpreter {
    demo: Demo,
    state: UnsafeCell<InterpreterState>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

pub struct InterpreterContext<'a> {
    pub renderer: &'a mut Renderer,
    pub render_target: &'a wgpu::TextureView,
    pub frame: f32,
}

impl Interpreter {
    pub fn new(demo: Demo) -> Self {
        let nodes_count = demo.graph.nodes.len();
        let evaluators = demo
            .graph
            .nodes
            .iter()
            .map(|n| n.operator.to_evaluator())
            .collect();

        Self {
            demo,
            state: UnsafeCell::new(InterpreterState {
                storage: OutputStorage::new(nodes_count),
                evaluators,
            }),
        }
    }

    pub fn run(&self, context: &mut InterpreterContext) -> Result<(), EvaluateError> {
        self.evaluate(context, NodeId::root())
    }

    pub(crate) fn evaluate(
        &self,
        context: &mut InterpreterContext,
        node_id: NodeId,
    ) -> Result<(), EvaluateError> {
        let state = self.state_mut();

        let mut evaluate_context = EvaluateContext {
            node_id,
            interpreter: self,
            interpreter_context: context,
        };

        state
            .evaluators
            .get_mut(node_id.0)
            .unwrap()
            .evaluate(&mut evaluate_context)
    }

    pub(crate) fn write_output(&self, node_id: NodeId, port_id: PortId, value: Value) {
        self.state_mut()
            .storage
            .write_output(node_id, port_id, value)
    }

    pub(crate) fn evaluate_input(
        &self,
        context: &mut InterpreterContext,
        node_id: NodeId,
        port_id: PortId,
    ) -> Result<Value, EvaluateError> {
        let node = self.demo.graph.get_node(node_id).unwrap();
        let input_state = node.get_input_state(port_id).unwrap();

        match input_state {
            InputState::Constant(c) => match c {
                Constant::Scalar(c) => Ok(Value::Scalar(*c)),
                Constant::I32(_) => todo!(),
                Constant::Vector(c) => Ok(Value::Vector3(*c)),
            },
            InputState::Connection(cs) => {
                let connection = cs.first().ok_or_else(|| EvaluateError::GenericError)?;

                self.evaluate(context, connection.get_node_id())?;

                let value = self
                    .state_mut()
                    .storage
                    .get_output(connection.get_node_id(), connection.get_port_id());

                Ok(value)
            }
        }
    }

    pub(crate) fn evaluate_input_multiple(
        &self,
        context: &mut InterpreterContext,
        node_id: NodeId,
        port_id: PortId,
    ) -> Result<MultipleValue, EvaluateError> {
        let node = self.demo.graph.get_node(node_id).unwrap();
        let input_state = node.get_input_state(port_id).unwrap();

        match input_state {
            InputState::Constant(c) => Err(EvaluateError::GenericError),
            InputState::Connection(cs) => {
                for c in cs {
                    self.evaluate(context, c.get_node_id())?;
                }

                let values = cs
                    .iter()
                    .map(|output| {
                        self.state_mut()
                            .storage
                            .get_output(output.get_node_id(), output.get_port_id())
                    })
                    .collect();

                Ok(MultipleValue { values })
            }
        }
    }

    fn state_mut(&self) -> &mut InterpreterState {
        /// SAFETY: The interpreter is largely immutable.
        /// The only places where mutation is possible is inside each Evaluator
        /// and inside the OutputStorage, which none break the interpreter.
        unsafe {
            &mut *self.state.get()
        }
    }
}

unsafe impl Send for Interpreter {}

unsafe impl Sync for Interpreter {}
