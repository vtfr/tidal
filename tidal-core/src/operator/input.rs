use std::any::{Any, TypeId};
use std::fmt::{Debug, Formatter};

use dyn_clone::{clone_trait_object, DynClone};
use smallbox::space::{S4, S8};
use smallbox::{smallbox, SmallBox};

use crate::graph::graph::ValuePlacement;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InputId(pub usize);

pub trait InputSlot: DynClone {
    /// Assigns a [dynamic] value to the slot using the provided [placement].
    ///
    /// [dynamic]: Any
    /// [placement]: ValuePlacement
    fn assign_constant_dyn(&mut self, placement: ValuePlacement, value: &dyn Any);

    /// Connects another [Node]'s output to this slot using the provided [placement].
    ///
    /// [Node]: crate::graph::node::Node
    /// [placement]: ValuePlacement
    fn connect(&mut self, placement: ValuePlacement, output: ());

    /// Disconnects at position index
    fn disconnect(&mut self, index: usize);

    /// Returns the [operator]'s inner [type id].
    ///
    /// [operator]: crate::operator::operator::Operator
    /// [type id]: TypeId
    fn inner_type_id(&self) -> TypeId;
}

#[derive(Debug, Clone)]
pub enum InputSource<T> {
    Constant(T),
    Connections(),
}

#[derive(Clone)]
pub struct Input<T> {
    value: InputSource<T>,
    default: Option<T>,
}

impl<T> Input<T>
where
    T: Default + Clone + Sized + 'static,
{
    pub fn new(default: Option<T>) -> Self {
        Self {
            value: InputSource::Constant(default.clone().unwrap_or_default()),
            default,
        }
    }

    /// Assigns a constant value [`T`] to the slot using the provided [placement].
    ///
    /// [placement]: ValuePlacement
    pub fn assign_constant(&mut self, _: ValuePlacement, value: T) {
        self.value = InputSource::Constant(value)
    }

    fn disconnect(&mut self, _: usize) {}

    #[inline]
    pub fn assign_constant_dyn(&mut self, placement: ValuePlacement, value: &dyn Any) {}
}

impl<T> InputSlot for Input<T>
where
    T: 'static + Default + Clone,
{
    fn assign_constant_dyn(&mut self, placement: ValuePlacement, value: &dyn Any) {
        if let Some(typed_value) = value.downcast_ref::<T>() {
            self.assign_constant(placement, typed_value.clone());
        }
    }

    fn connect(&mut self, placement: ValuePlacement, output: ()) {
        todo!()
    }

    fn disconnect(&mut self, index: usize) {
        todo!()
    }

    fn inner_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}
