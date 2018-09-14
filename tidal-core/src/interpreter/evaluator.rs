use cgmath::Vector3;

use tidal_core_derive::evaluator;

use crate::graph::{NodeId, PortId};
use crate::interpreter::{Interpreter, InterpreterContext, Multiple, MultipleValue, Value};
use crate::operator::Operator;
use crate::renderer::Renderer;

#[derive(Copy, Clone, Debug)]
pub enum EvaluateError {
    GenericError,
}

pub trait Evaluate {
    fn evaluate(&mut self, ctx: &mut EvaluateContext) -> Result<(), EvaluateError>;
}

pub struct EvaluateContext<'a, 'i> {
    pub node_id: NodeId,
    pub interpreter: &'i Interpreter,
    pub interpreter_context: &'i mut InterpreterContext<'a>,
}

impl<'a, 'i> EvaluateContext<'a, 'i> {
    #[inline(always)]
    pub(crate) fn evaluate_input(
        &mut self,
        port_id: impl Into<PortId>,
    ) -> Result<Value, EvaluateError> {
        self.interpreter
            .evaluate_input(self.interpreter_context, self.node_id, port_id.into())
    }

    #[inline(always)]
    pub(crate) fn evaluate_input_multiple<T>(
        &mut self,
        port_id: impl Into<PortId>,
    ) -> Result<MultipleValue, EvaluateError> {
        self.interpreter.evaluate_input_multiple(
            self.interpreter_context,
            self.node_id,
            port_id.into(),
        )
    }

    #[inline(always)]
    pub(crate) fn write_output(&mut self, port_id: impl Into<PortId>, value: Value) {
        self.interpreter
            .write_output(self.node_id, port_id.into(), value.into())
    }

    #[inline(always)]
    pub(crate) fn renderer(&mut self) -> &mut Renderer {
        self.interpreter_context.renderer
    }
}
