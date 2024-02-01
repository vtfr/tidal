use std::any::Any;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::Component;

use thiserror::Error;

use crate::engine::di::Container;
use crate::engine::tick::Tick;
use crate::graph::graph::Graph;
use crate::graph::node::{Node, NodeId};
use crate::operator::output::{OutputId, OutputSlot};

#[derive(Default, Copy, Clone)]
pub struct GraphRunner;

impl GraphRunner {
    pub fn run(&self, container: &Container) {
        let Some(graph) = container.get_ref::<Graph>() else {
            return;
        };

        let Some(tick) = container.get_ref::<Tick>().copied() else {
            return;
        };

        let context = Context {
            graph: SafeGraphCell::of(graph),
            tick,
            container,
        };

        context.evaluate(NodeId::new(0));
    }
}

/// Stores contextual information about the current [`Graph`] evaluation.
pub struct Context<'a> {
    graph: SafeGraphCell<'a>,
    tick: Tick,
    container: &'a Container,
}

impl<'a> Context<'a> {
    #[inline]
    pub fn container(&self) -> &'a Container {
        self.container
    }

    pub fn get_output_slot(&self, node_id: NodeId, output_id: OutputId) -> Option<&dyn OutputSlot> {
        self.evaluate(node_id);

        match self.graph.get_node_ref(node_id) {
            Ok(Some(node)) => node.operator().get_output(output_id),
            Ok(None) => {
                /// Node not found. Let caller ([`InputSlot`]) decide how to proceed.
                None
            }
            Err(SafeGraphCellNodeError::CycleDetected) => cycle_detected(node_id),
        }
    }

    #[inline]
    pub(crate) fn evaluate(&self, node_id: NodeId) {
        match self.graph.get_node_mut(node_id) {
            Ok(Some(mut node)) => {
                // Only evaluate nodes once per tick
                if node.last_evaluated_at() != self.tick {
                    node.start_evaluating();
                    node.operator_mut().run(self);
                    node.finish_evaluating(self.tick);

                    debug_assert_eq!(node.last_evaluated_at(), self.tick)
                }
            }
            Ok(None) => {}
            Err(SafeGraphCellNodeError::CycleDetected) => cycle_detected(node_id),
        }
    }
}

#[inline(always)]
fn cycle_detected(node_id: NodeId) -> ! {
    panic!("cycle detected while trying to evaluate node {:?}", node_id)
}

/// A safe [`Graph`] cell that ensures [`Operator`]s are only able to mutate the graph in safe ways.
///
/// The [`Graph`] invariant rules automatically ensure the graph is valid and contains no cycles.
/// However, other issues could arise from broken [`InputSlot`] implementations, such as creating
/// invalid references to other node's outputs by getting their value and reevaluating the child
/// node.
///
/// To mitigate that, we track all the operators that have been evaluated for the current frame
/// and prevent them from being re-evaluated again.
///
/// As a side-effect of that, we can also prevent operators from being evaluated multiple times
/// on a single frame, resulting in a performance boost for operators with multiple outputs.
///
/// [`InputSlot`]: crate::operator::input::InputSlot
/// [`Operator`]: crate::operator::operator::Operator
#[derive(Copy, Clone)]
pub struct SafeGraphCell<'graph>(*const Graph, PhantomData<&'graph Graph>);

impl<'graph> SafeGraphCell<'graph> {
    pub fn of(graph: &'graph Graph) -> SafeGraphCell {
        Self(graph as *const _, PhantomData)
    }
}

#[derive(Debug, Copy, Clone, Error)]
pub enum SafeGraphCellNodeError {
    #[error("attempting to evaluate a node that already is being evaluated (cycle)")]
    CycleDetected,
}

impl<'graph> SafeGraphCell<'graph> {
    fn get_node_ref(&self, node_id: NodeId) -> Result<Option<&Node>, SafeGraphCellNodeError> {
        // SAFETY: we'll only read from the graph after we've determined we have exclusive access
        // to the node.
        let graph = unsafe { self.as_ref() };

        match graph.get_node(node_id) {
            Some(node) => {
                if node.evaluating() {
                    Err(SafeGraphCellNodeError::CycleDetected)
                } else {
                    Ok(Some(node))
                }
            }
            None => Ok(None),
        }
    }

    fn get_node_mut(&self, node_id: NodeId) -> Result<Option<&mut Node>, SafeGraphCellNodeError> {
        // SAFETY: we'll only modify the graph after we've determined we have exclusive access
        // to the node.
        let graph = unsafe { self.as_mut() };

        match graph.get_node_mut(node_id) {
            Some(node) => {
                if node.evaluating() {
                    Err(SafeGraphCellNodeError::CycleDetected)
                } else {
                    Ok(Some(node))
                }
            }
            None => Ok(None),
        }
    }

    /// Safety: callers must ensure safe access.
    #[inline]
    unsafe fn as_mut(&self) -> &mut Graph {
        &mut *(self.0 as *mut Graph)
    }

    /// Safety: callers must ensure safe access.
    #[inline]
    unsafe fn as_ref(&self) -> &Graph {
        &*self.0
    }
}
