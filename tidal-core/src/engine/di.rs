use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use crate::all_tuples;

/// A dependency that can be fetched from the [engine].
///
/// [engine]: crate::engine::engine::Engine
pub trait DependencyResolver {
    /// The resolved dependency associated with the call site's lifetime.
    type Item<'op>;

    fn resolve(container: &Container) -> Self::Item<'_>;
}

// Implementation for types that are sent via a reference.
impl<T> DependencyResolver for &'_ T
where
    T: 'static,
{
    type Item<'op> = &'op T;

    #[inline(always)]
    fn resolve(container: &Container) -> Self::Item<'_> {
        match container.get_ref::<T>() {
            Some(resource) => resource,
            None => panic!(
                "unresolvable resource of type {:?}",
                std::any::type_name::<T>()
            ),
        }
    }
}

pub trait DependenciesResolver {
    type Item<'op>;

    fn resolve_all(container: &Container) -> Self::Item<'_>;
}

impl DependenciesResolver for () {
    type Item<'op> = ();

    #[inline(always)]
    fn resolve_all(_: &Container) -> Self::Item<'_> {
        ()
    }
}

pub type Resolved<'a, Deps: DependenciesResolver> = (Deps::Item<'a>);

macro_rules! impl_dependencies_resolver {
    ($($ty:ident),*) => {
        impl<$($ty),*> DependenciesResolver for ($($ty),*,)
        where
            $(
                $ty: DependencyResolver
            ),*
        {
            type Item<'op> = ($(<$ty>::Item<'op>),*,);

            #[inline(always)]
            fn resolve_all(container: &Container) -> Self::Item<'_> {
                (
                    $(
                        <$ty>::resolve(container)
                    ),*,
                )
            }
        }
    };
}

all_tuples!(impl_dependencies_resolver);

/// A dependency injection container.
#[derive(Default)]
pub struct Container {
    inner: HashMap<TypeId, Box<dyn Any>>,
}

impl Debug for Container {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Container({:?}", self.inner.keys()))
    }
}

impl Container {
    #[inline]
    pub fn register<T>(&mut self, resource: T)
    where
        T: Any + 'static,
    {
        let value = Box::new(resource) as Box<dyn Any>;

        self.inner.insert(TypeId::of::<T>(), value);
    }

    #[inline]
    pub fn is_registered<T>(&self) -> bool
    where
        T: Any + 'static,
    {
        self.inner.contains_key(&TypeId::of::<T>())
    }

    #[inline]
    pub fn get_ref<T>(&self) -> Option<&T>
    where
        T: Any + 'static,
    {
        self.inner
            .get(&TypeId::of::<T>())
            .map(resource_cast_ref::<T>)
    }

    #[inline]
    pub fn get_cloned<T>(&self) -> Option<T>
    where
        T: Any + Clone + 'static,
    {
        self.get_ref::<T>().cloned()
    }

    #[inline]
    pub fn get_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Any + 'static,
    {
        self.inner
            .get_mut(&TypeId::of::<T>())
            .map(resource_cast_mut::<T>)
    }

    #[inline]
    pub fn get_mut_or_default<T>(&mut self) -> &mut T
    where
        T: Default + Any + 'static,
    {
        let boxed = self
            .inner
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(T::default()));

        resource_cast_mut(boxed)
    }
}

#[inline(always)]
fn resource_cast_ref<T>(resource: &Box<dyn Any>) -> &T
where
    T: Any + 'static,
{
    /// SAFETY: only values of type T can be inserted the Container with the key
    /// TypeId::of::<T>(), so we can skip the secondary check.
    unsafe {
        (&*resource).downcast_ref_unchecked::<T>()
    }
}

#[inline(always)]
fn resource_cast_mut<T>(resource: &mut Box<dyn Any>) -> &mut T
where
    T: Any + 'static,
{
    /// SAFETY: only values of type T can be inserted the Container with the key
    /// TypeId::of::<T>(), so we can skip the secondary check.
    unsafe {
        (&mut *resource).downcast_mut_unchecked::<T>()
    }
}
