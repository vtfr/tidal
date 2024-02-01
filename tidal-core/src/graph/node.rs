use std::any::{Any, TypeId};
use std::fmt::{Debug, Formatter};
use std::num::NonZeroU32;
use std::slice::{Iter, IterMut};

use serde::{Deserialize, Serialize, Serializer};

use crate::engine::tick::Tick;
use crate::graph::runner::Context;
use crate::operator::operator::{
    MaterializedOperator, MaterializedOperatorImpl, Metadata, Operator, OperatorId,
};
use crate::operator::output::{OutputId, OutputSlot};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(transparent)]
pub(crate) struct Generation(NonZeroU32);

impl PartialEq for Generation {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.to_generation_bits() == other.to_generation_bits()
    }
}

impl Generation {
    /// The [`VACANT_FLAG`] is used as a to flag generations that are vacant.
    const VACANT_FLAG: u32 = 1 << (u32::BITS - 1);
    const MASK: u32 = !Self::VACANT_FLAG;

    const MIN: Generation = Self(NonZeroU32::MIN);

    /// Extracts the generation bits from the [`Generation`], excluding flags.
    #[inline(always)]
    pub const fn to_generation_bits(self) -> u32 {
        self.0.get() & Self::MASK
    }

    /// Tags this [`Generation`] as [`vacant`].
    ///
    /// [`vacant`]: Generation::VACANT_FLAG
    #[inline]
    pub const fn into_vacant(self) -> Self {
        let value = self.0.get() | Self::VACANT_FLAG;
        let inner = unsafe { NonZeroU32::new_unchecked(value) };
        Generation(inner)
    }

    pub const fn is_vacant(&self) -> bool {
        self.0.get() & Self::VACANT_FLAG == Self::VACANT_FLAG
    }

    /// Increments the [`Generation`]. If this Generation is tagged as [`vacant`],
    /// incrementing it will automatically remove the vacant bit.
    ///
    /// [`vacant`]: Generation::VACANT_FLAG
    pub fn increment(self) -> Self {
        self.to_generation_bits()
            .checked_add(1)
            .and_then(|v| {
                // Ensure we're not overflowing the mask.
                ((v & Self::MASK) == v).then_some(v)
            })
            .and_then(|v| {
                let value = unsafe { NonZeroU32::new_unchecked(v) };
                Some(Generation(value))
            })
            .expect("too many generations")
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C, align(8))]
pub struct NodeId {
    index: u32,
    generation: Generation,
}

impl NodeId {
    pub(crate) const fn new(index: u32) -> Self {
        Self {
            index,
            generation: Generation::MIN,
        }
    }

    /// Converts this [`NodeId`] to a vacant [`NodeId`].
    #[inline]
    pub(crate) const fn into_vacant(self) -> Self {
        Self {
            generation: self.generation.into_vacant(),
            ..self
        }
    }

    #[inline]
    pub(crate) const fn is_vacant(self) -> bool {
        self.generation.is_vacant()
    }

    #[inline]
    pub(crate) const fn index(&self) -> usize {
        self.index as usize
    }

    #[inline]
    pub(crate) const fn generation(&self) -> Generation {
        self.generation
    }

    #[inline]
    pub(crate) fn increment_generation(self) -> Self {
        Self {
            generation: self.generation.increment(),
            ..self
        }
    }
}

#[cfg(test)]
mod test {
    use super::NodeId;

    #[test]
    pub fn test_node_id_memory_alignment() {
        assert_eq!(std::mem::size_of::<NodeId>(), std::mem::size_of::<u64>())
    }

    #[test]
    pub fn test_option_optimization() {
        assert_eq!(
            std::mem::size_of::<NodeId>(),
            std::mem::size_of::<Option<NodeId>>()
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NodeEntry {
    id: NodeId,
    node: Node,
}

impl NodeEntry {
    #[inline]
    pub(crate) fn empty(id: NodeId) -> Self {
        todo!()
        // Self {
        //     id,
        //     node: Default::default(),
        // }
    }

    pub fn is_vacant(&self) -> bool {
        self.id.generation().is_vacant()
    }

    pub fn is_occupied(&self) -> bool {
        !self.id.generation().is_vacant()
    }

    pub fn operator_storage_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

#[derive(Default, Debug, Clone)]
pub struct Nodes {
    nodes: Vec<NodeEntry>,
    free_list: Vec<usize>,
}

impl Nodes {
    /// Retrieves the root [`Node`].
    ///
    /// The root node is a special node that is used by the [`Graph`] to start evaluation. It's
    /// always stored as the first node in the [`Nodes`] storage.
    #[inline]
    pub fn root(&self) -> Option<&Node> {
        self.nodes.first().map(|e| &e.node)
    }

    /// Retrieves a [`Node`] based on it's id, checking if their generations match.
    pub fn get(&self, id: NodeId) -> Option<&Node> {
        let index = id.index();

        self.nodes
            .get(index)
            .filter(|entry| entry.is_occupied())
            .and_then(|entry: &NodeEntry| (entry.id == id).then_some(&entry.node))
    }

    /// Retrieves a mutable [`NodeEntry`] based on it's id, checking if their generations match.
    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        let index = id.index();

        self.nodes
            .get_mut(index)
            .filter(|entry| entry.is_occupied())
            .and_then(|entry| (entry.id == id).then_some(&mut entry.node))
    }

    pub fn remove(&mut self, id: NodeId) {
        let index = id.index();

        if let Some(entry) = self.nodes.get_mut(index) {
            if entry.is_occupied() {
                *entry = NodeEntry::empty(id.into_vacant());
                self.free_list.push(index);
            }
        }
    }

    pub fn add(&mut self, node: Node) -> NodeId {
        if let Some(free_index) = self.free_list.pop() {
            let replace = self.nodes.get_mut(free_index).unwrap();

            let id = replace.id.increment_generation();

            *replace = NodeEntry { id, node };

            id
        } else {
            let index = u32::try_from(self.nodes.len()).expect("too many nodes");
            let id = NodeId::new(index);
            self.nodes.push(NodeEntry { id, node });

            id
        }
    }

    #[inline]
    pub fn iter(&self) -> NodeIter {
        NodeIter {
            inner: self.nodes.iter(),
        }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> NodeIterMut {
        NodeIterMut {
            inner: self.nodes.iter_mut(),
        }
    }
}

pub struct NodeIter<'a> {
    pub(crate) inner: Iter<'a, NodeEntry>,
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = (NodeId, &'a Node);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.inner.next() {
            if entry.is_occupied() {
                return Some((entry.id, &entry.node));
            }
        }

        None
    }
}

pub struct NodeIterMut<'a> {
    pub(crate) inner: IterMut<'a, NodeEntry>,
}

impl<'a> Iterator for NodeIterMut<'a> {
    type Item = (NodeId, &'a mut Node);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.inner.next() {
            if entry.is_occupied() {
                return Some((entry.id, &mut entry.node));
            }
        }

        None
    }
}

#[derive(Clone)]
pub struct Node {
    id: NodeId,
    operator: Box<dyn MaterializedOperator>,

    /// Keeps track of when was this node last evaluated.
    last_evaluated_at: Tick,

    /// Tracks whether we're evaluating the current node.
    evaluating: bool,
}

impl Node {
    pub fn start_evaluating(&mut self) {
        self.evaluating = true;
    }

    pub fn finish_evaluating(&mut self, tick: Tick) {
        self.evaluating = false;
        self.last_evaluated_at = tick;
    }

    #[inline]
    pub fn evaluating(&self) -> bool {
        self.evaluating
    }

    #[inline]
    pub fn last_evaluated_at(&self) -> Tick {
        self.last_evaluated_at
    }

    #[inline]
    pub fn operator_mut(&mut self) -> &mut dyn MaterializedOperator {
        &mut *self.operator
    }

    #[inline]
    pub fn operator(&self) -> &dyn MaterializedOperator {
        &*self.operator
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Node({:?}))", self.operator.id()))
    }
}

#[derive(Copy, Clone)]
pub struct Noop;

const NOOP_METADATA: Metadata = Metadata {
    name: "noop",
    inputs: &[],
    outputs: &[],
};

impl MaterializedOperator for Noop {
    fn run(&mut self, _: &Context) {}

    fn metadata(&self) -> &'static Metadata {
        &NOOP_METADATA
    }

    fn id(&self) -> OperatorId {
        OperatorId(TypeId::of::<Noop>())
    }

    fn get_output(&self, _: OutputId) -> Option<&dyn OutputSlot> {
        None
    }
}
