use std::any::{Any, TypeId};
use std::fmt::{Debug, Formatter};

use dyn_clone::{clone_trait_object, DynClone};
use smallbox::space::{S4, S8};
use smallbox::{smallbox, SmallBox};

use crate::graph::graph::ValuePlacement;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OutputId(pub usize);

pub trait OutputSlot: Any + DynClone {
    /// Writes a value of a [dynamic] type, if this value conforms
    /// to the output's inner type.
    ///
    /// [dynamic]: Any fn write_dyn(&mut self, value: &dyn Any); fn get_dyn(&mut self) -> Option<&dyn Any>;
    fn write_dyn(&mut self, value: &dyn Any);

    fn get_dyn(&mut self) -> Option<&dyn Any>;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

clone_trait_object!(OutputSlot);

impl dyn OutputSlot {
    pub fn cast<T>(&self) -> Option<&TypedOutputSlot<T>>
    where
        T: 'static,
    {
        self.as_any().downcast_ref::<TypedOutputSlot<T>>()
    }

    pub fn cast_mut<T>(&mut self) -> Option<&mut TypedOutputSlot<T>>
    where
        T: 'static,
    {
        self.as_any_mut().downcast_mut::<TypedOutputSlot<T>>()
    }
}

#[derive(Clone)]
pub struct OutputState {
    slot: BoxedOutputSlot,
}

#[derive(Clone)]
pub struct TypedOutputSlot<T>(Option<T>);

pub type BoxedOutputSlot = Box<dyn OutputSlot>;

impl<T> TypedOutputSlot<T>
where
    T: Clone + Default + Sized + 'static,
{
    pub const fn new() -> Self {
        Self(None)
    }

    #[inline]
    pub fn get(&mut self) -> Option<&T> {
        self.0.as_ref()
    }

    /// Assigns a constant value [`T`] to the slot using the provided [placement].
    ///
    /// [placement]: ValuePlacement
    pub fn write(&mut self, value: T) {
        self.0 = Some(value);
    }

    pub fn into_boxed(self) -> BoxedOutputSlot {
        Box::new(self)
    }
}

impl<T> OutputSlot for TypedOutputSlot<T>
where
    T: Clone + Default + Any,
{
    fn write_dyn(&mut self, value: &dyn Any) {
        if let Some(value) = value.downcast_ref::<T>() {
            TypedOutputSlot::write(self, value.clone())
        }
    }

    fn get_dyn(&mut self) -> Option<&dyn Any> {
        self.get().map(|v| v as &dyn Any)
    }

    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}
