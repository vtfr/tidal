use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter, Write};

use dyn_clone::{clone_trait_object, DynClone};

use crate::engine::di::{DependenciesResolver, Resolved};
use crate::graph::node::Node;
use crate::graph::runner::Context;
use crate::operator::output::{OutputId, OutputSlot};

/// [`Operator`] is the base building block of Vereis.
///
/// Operators are stateful or stateless functions that operate on a given set of inputs and
/// produce outputs that can be connected to other Operators or displayed on the screen.
pub trait Operator: Clone + Evaluate + Sized {
    /// Instantiates an operator.
    fn new() -> Self;

    /// Provides [`Metadata`] for this operator.
    fn metadata() -> &'static Metadata;

    fn get_output(&self, id: OutputId) -> Option<&dyn OutputSlot>;
}

/// Something that can be evaluated.
pub trait Evaluate {
    type Deps: DependenciesResolver;

    fn evaluate(&mut self, args: Resolved<Self::Deps>);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct OperatorId(pub(crate) TypeId);

impl OperatorId {
    pub fn of<O>() -> Self
    where
        O: Operator + 'static,
    {
        Self(TypeId::of::<O>())
    }
}

pub struct InputMetadata {
    pub name: &'static str,
}

pub struct OutputMetadata {
    pub name: &'static str,
}

pub struct Metadata {
    pub name: &'static str,
    pub inputs: &'static [InputMetadata],
    pub outputs: &'static [OutputMetadata],
}

/// Type-erased version of an [`Operator`] that handler it's own data fetching and storing
pub trait MaterializedOperator: DynClone {
    /// Runs this operator against a given [node]'s [storage].
    ///
    /// [node]: Node
    /// [storage]: NodeStorage
    fn run(&mut self, context: &Context);

    /// Returns this operator's metadata.
    fn metadata(&self) -> &'static Metadata;

    /// Returns the Operator's internal ID.
    fn id(&self) -> OperatorId;

    fn get_output(&self, id: OutputId) -> Option<&dyn OutputSlot>;
}

clone_trait_object!(MaterializedOperator);

#[derive(Clone)]
pub struct MaterializedOperatorImpl<O>(O);

/// Default implementation of a [`MaterializedOperator`], performing the
/// dependency resolution and calling the [evaluate] method.
///
/// [evaluate]: Evaluate::evaluate
impl<O> MaterializedOperatorImpl<O>
where
    O: Operator + Evaluate + Sized + Clone,
{
    pub fn new(operator: O) -> Self {
        Self(operator)
    }
}

impl<O> MaterializedOperator for MaterializedOperatorImpl<O>
where
    O: Operator + Evaluate + Sized + Clone + 'static,
{
    fn run(&mut self, context: &Context) {
        let deps = O::Deps::resolve_all(context.container());

        self.0.evaluate(deps)
    }

    fn metadata(&self) -> &'static Metadata {
        O::metadata()
    }

    fn id(&self) -> OperatorId {
        OperatorId::of::<O>()
    }

    fn get_output(&self, id: OutputId) -> Option<&dyn OutputSlot> {
        self.0.get_output(id)
    }
}
