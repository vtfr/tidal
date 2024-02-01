pub mod function {
    use std::any::{Any, TypeId};
    use std::borrow::Cow;
    use std::env::Args;
    use std::marker::PhantomData;

    use crate::all_tuples;
    use crate::engine::instance::{InputSlots, InputState, Node, NodeOperatorStorage};
    use crate::engine::operator::Operator;
    use crate::engine::OperatorSafeEngineCell;

    pub struct NodeId(u64);

    /// Initializes a slot for a accessor.
    /// This is used for implementing input slots,
    /// such as [`inputSingleSlot`].
    pub trait InitInputState {
        fn init_input_state() -> InputState;
    }

    /// Initializes the state to it's default value.
    pub trait InitState: Default {
        fn init_state() -> Option<Box<dyn Any>>;
    }

    impl<T> InitState for T
    where
        T: Default + 'static,
    {
        fn init_state() -> Option<Box<dyn Any>> {
            let def: T = Default::default();
            Some(Box::new(def))
        }
    }

    impl<T> InitInputState for T
    where
        T: Default + 'static,
    {
        fn init_input_state() -> InputState {
            InputState {
                data_type: TypeId::of::<T>(),
                slots: InputSlots::<T>::new_with_default().into_boxed(),
                multiple: false,
            }
        }
    }

    /// Initializes the input state with a given type.
    pub trait InitInputStates {
        fn init_input_states() -> Vec<InputState>;
    }

    impl<A1> InitInputStates for (A1,)
    where
        A1: InitInputState + Default + 'static,
    {
        fn init_input_states() -> Vec<InputState> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([A1::init_input_state()]),
            )
        }
    }

    impl<A1, A2> InitInputStates for (A1, A2)
    where
        A1: InitInputState + Default + 'static,
        A2: InitInputState + Default + 'static,
    {
        fn init_input_states() -> Vec<InputState> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([A1::init_input_state(), A2::init_input_state()]),
            )
        }
    }

    impl<A1, A2, A3> InitInputStates for (A1, A2, A3)
    where
        A1: InitInputState + Default + 'static,
        A2: InitInputState + Default + 'static,
        A3: InitInputState + Default + 'static,
    {
        fn init_input_states() -> Vec<InputState> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    A1::init_input_state(),
                    A2::init_input_state(),
                    A3::init_input_state(),
                ]),
            )
        }
    }

    impl<A1, A2, A3, A4> InitInputStates for (A1, A2, A3, A4)
    where
        A1: InitInputState + Default + 'static,
        A2: InitInputState + Default + 'static,
        A3: InitInputState + Default + 'static,
        A4: InitInputState + Default + 'static,
    {
        fn init_input_states() -> Vec<InputState> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    A1::init_input_state(),
                    A2::init_input_state(),
                    A3::init_input_state(),
                    A4::init_input_state(),
                ]),
            )
        }
    }

    impl<A1, A2, A3, A4, A5> InitInputStates for (A1, A2, A3, A4, A5)
    where
        A1: InitInputState + Default + 'static,
        A2: InitInputState + Default + 'static,
        A3: InitInputState + Default + 'static,
        A4: InitInputState + Default + 'static,
        A5: InitInputState + Default + 'static,
    {
        fn init_input_states() -> Vec<InputState> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    A1::init_input_state(),
                    A2::init_input_state(),
                    A3::init_input_state(),
                    A4::init_input_state(),
                    A5::init_input_state(),
                ]),
            )
        }
    }

    impl<A1, A2, A3, A4, A5, A6> InitInputStates for (A1, A2, A3, A4, A5, A6)
    where
        A1: InitInputState + Default + 'static,
        A2: InitInputState + Default + 'static,
        A3: InitInputState + Default + 'static,
        A4: InitInputState + Default + 'static,
        A5: InitInputState + Default + 'static,
        A6: InitInputState + Default + 'static,
    {
        fn init_input_states() -> Vec<InputState> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    A1::init_input_state(),
                    A2::init_input_state(),
                    A3::init_input_state(),
                    A4::init_input_state(),
                    A5::init_input_state(),
                    A6::init_input_state(),
                ]),
            )
        }
    }

    impl<A1, A2, A3, A4, A5, A6, A7> InitInputStates for (A1, A2, A3, A4, A5, A6, A7)
    where
        A1: InitInputState + Default + 'static,
        A2: InitInputState + Default + 'static,
        A3: InitInputState + Default + 'static,
        A4: InitInputState + Default + 'static,
        A5: InitInputState + Default + 'static,
        A6: InitInputState + Default + 'static,
        A7: InitInputState + Default + 'static,
    {
        fn init_input_states() -> Vec<InputState> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    A1::init_input_state(),
                    A2::init_input_state(),
                    A3::init_input_state(),
                    A4::init_input_state(),
                    A5::init_input_state(),
                    A6::init_input_state(),
                    A7::init_input_state(),
                ]),
            )
        }
    }

    impl<A1, A2, A3, A4, A5, A6, A7, A8> InitInputStates for (A1, A2, A3, A4, A5, A6, A7, A8)
    where
        A1: InitInputState + Default + 'static,
        A2: InitInputState + Default + 'static,
        A3: InitInputState + Default + 'static,
        A4: InitInputState + Default + 'static,
        A5: InitInputState + Default + 'static,
        A6: InitInputState + Default + 'static,
        A7: InitInputState + Default + 'static,
        A8: InitInputState + Default + 'static,
    {
        fn init_input_states() -> Vec<InputState> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    A1::init_input_state(),
                    A2::init_input_state(),
                    A3::init_input_state(),
                    A4::init_input_state(),
                    A5::init_input_state(),
                    A6::init_input_state(),
                    A7::init_input_state(),
                    A8::init_input_state(),
                ]),
            )
        }
    }

    impl<A1, A2, A3, A4, A5, A6, A7, A8, A9> InitInputStates for (A1, A2, A3, A4, A5, A6, A7, A8, A9)
    where
        A1: InitInputState + Default + 'static,
        A2: InitInputState + Default + 'static,
        A3: InitInputState + Default + 'static,
        A4: InitInputState + Default + 'static,
        A5: InitInputState + Default + 'static,
        A6: InitInputState + Default + 'static,
        A7: InitInputState + Default + 'static,
        A8: InitInputState + Default + 'static,
        A9: InitInputState + Default + 'static,
    {
        fn init_input_states() -> Vec<InputState> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    A1::init_input_state(),
                    A2::init_input_state(),
                    A3::init_input_state(),
                    A4::init_input_state(),
                    A5::init_input_state(),
                    A6::init_input_state(),
                    A7::init_input_state(),
                    A8::init_input_state(),
                    A9::init_input_state(),
                ]),
            )
        }
    }

    impl<A1, A2, A3, A4, A5, A6, A7, A8, A9, A10> InitInputStates
        for (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10)
    where
        A1: InitInputState + Default + 'static,
        A2: InitInputState + Default + 'static,
        A3: InitInputState + Default + 'static,
        A4: InitInputState + Default + 'static,
        A5: InitInputState + Default + 'static,
        A6: InitInputState + Default + 'static,
        A7: InitInputState + Default + 'static,
        A8: InitInputState + Default + 'static,
        A9: InitInputState + Default + 'static,
        A10: InitInputState + Default + 'static,
    {
        fn init_input_states() -> Vec<InputState> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    A1::init_input_state(),
                    A2::init_input_state(),
                    A3::init_input_state(),
                    A4::init_input_state(),
                    A5::init_input_state(),
                    A6::init_input_state(),
                    A7::init_input_state(),
                    A8::init_input_state(),
                    A9::init_input_state(),
                    A10::init_input_state(),
                ]),
            )
        }
    }

    impl<A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11> InitInputStates
        for (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11)
    where
        A1: InitInputState + Default + 'static,
        A2: InitInputState + Default + 'static,
        A3: InitInputState + Default + 'static,
        A4: InitInputState + Default + 'static,
        A5: InitInputState + Default + 'static,
        A6: InitInputState + Default + 'static,
        A7: InitInputState + Default + 'static,
        A8: InitInputState + Default + 'static,
        A9: InitInputState + Default + 'static,
        A10: InitInputState + Default + 'static,
        A11: InitInputState + Default + 'static,
    {
        fn init_input_states() -> Vec<InputState> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    A1::init_input_state(),
                    A2::init_input_state(),
                    A3::init_input_state(),
                    A4::init_input_state(),
                    A5::init_input_state(),
                    A6::init_input_state(),
                    A7::init_input_state(),
                    A8::init_input_state(),
                    A9::init_input_state(),
                    A10::init_input_state(),
                    A11::init_input_state(),
                ]),
            )
        }
    }

    impl<A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12> InitInputStates
        for (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12)
    where
        A1: InitInputState + Default + 'static,
        A2: InitInputState + Default + 'static,
        A3: InitInputState + Default + 'static,
        A4: InitInputState + Default + 'static,
        A5: InitInputState + Default + 'static,
        A6: InitInputState + Default + 'static,
        A7: InitInputState + Default + 'static,
        A8: InitInputState + Default + 'static,
        A9: InitInputState + Default + 'static,
        A10: InitInputState + Default + 'static,
        A11: InitInputState + Default + 'static,
        A12: InitInputState + Default + 'static,
    {
        fn init_input_states() -> Vec<InputState> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    A1::init_input_state(),
                    A2::init_input_state(),
                    A3::init_input_state(),
                    A4::init_input_state(),
                    A5::init_input_state(),
                    A6::init_input_state(),
                    A7::init_input_state(),
                    A8::init_input_state(),
                    A9::init_input_state(),
                    A10::init_input_state(),
                    A11::init_input_state(),
                    A12::init_input_state(),
                ]),
            )
        }
    }

    impl<A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13> InitInputStates
        for (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13)
    where
        A1: InitInputState + Default + 'static,
        A2: InitInputState + Default + 'static,
        A3: InitInputState + Default + 'static,
        A4: InitInputState + Default + 'static,
        A5: InitInputState + Default + 'static,
        A6: InitInputState + Default + 'static,
        A7: InitInputState + Default + 'static,
        A8: InitInputState + Default + 'static,
        A9: InitInputState + Default + 'static,
        A10: InitInputState + Default + 'static,
        A11: InitInputState + Default + 'static,
        A12: InitInputState + Default + 'static,
        A13: InitInputState + Default + 'static,
    {
        fn init_input_states() -> Vec<InputState> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    A1::init_input_state(),
                    A2::init_input_state(),
                    A3::init_input_state(),
                    A4::init_input_state(),
                    A5::init_input_state(),
                    A6::init_input_state(),
                    A7::init_input_state(),
                    A8::init_input_state(),
                    A9::init_input_state(),
                    A10::init_input_state(),
                    A11::init_input_state(),
                    A12::init_input_state(),
                    A13::init_input_state(),
                ]),
            )
        }
    }

    impl<A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14> InitInputStates
        for (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14)
    where
        A1: InitInputState + Default + 'static,
        A2: InitInputState + Default + 'static,
        A3: InitInputState + Default + 'static,
        A4: InitInputState + Default + 'static,
        A5: InitInputState + Default + 'static,
        A6: InitInputState + Default + 'static,
        A7: InitInputState + Default + 'static,
        A8: InitInputState + Default + 'static,
        A9: InitInputState + Default + 'static,
        A10: InitInputState + Default + 'static,
        A11: InitInputState + Default + 'static,
        A12: InitInputState + Default + 'static,
        A13: InitInputState + Default + 'static,
        A14: InitInputState + Default + 'static,
    {
        fn init_input_states() -> Vec<InputState> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    A1::init_input_state(),
                    A2::init_input_state(),
                    A3::init_input_state(),
                    A4::init_input_state(),
                    A5::init_input_state(),
                    A6::init_input_state(),
                    A7::init_input_state(),
                    A8::init_input_state(),
                    A9::init_input_state(),
                    A10::init_input_state(),
                    A11::init_input_state(),
                    A12::init_input_state(),
                    A13::init_input_state(),
                    A14::init_input_state(),
                ]),
            )
        }
    }

    impl<A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15> InitInputStates
        for (
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        )
    where
        A1: InitInputState + Default + 'static,
        A2: InitInputState + Default + 'static,
        A3: InitInputState + Default + 'static,
        A4: InitInputState + Default + 'static,
        A5: InitInputState + Default + 'static,
        A6: InitInputState + Default + 'static,
        A7: InitInputState + Default + 'static,
        A8: InitInputState + Default + 'static,
        A9: InitInputState + Default + 'static,
        A10: InitInputState + Default + 'static,
        A11: InitInputState + Default + 'static,
        A12: InitInputState + Default + 'static,
        A13: InitInputState + Default + 'static,
        A14: InitInputState + Default + 'static,
        A15: InitInputState + Default + 'static,
    {
        fn init_input_states() -> Vec<InputState> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    A1::init_input_state(),
                    A2::init_input_state(),
                    A3::init_input_state(),
                    A4::init_input_state(),
                    A5::init_input_state(),
                    A6::init_input_state(),
                    A7::init_input_state(),
                    A8::init_input_state(),
                    A9::init_input_state(),
                    A10::init_input_state(),
                    A11::init_input_state(),
                    A12::init_input_state(),
                    A13::init_input_state(),
                    A14::init_input_state(),
                    A15::init_input_state(),
                ]),
            )
        }
    }

    /// General trait for building components that read from the engine.
    pub trait Fetch<S, I> {
        fn fetch(node: &Node, engine: OperatorSafeEngineCell) -> Self;
    }

    pub trait FetchAll<S, I> {
        fn fetch_all(node: &Node, engine: OperatorSafeEngineCell) -> Self;
    }

    impl<S, I, A1> FetchAll<S, I> for (A1,)
    where
        A1: Fetch<S, I>,
    {
        fn fetch_all(node: &Node, engine: OperatorSafeEngineCell) -> Self {
            (A1::fetch(node, engine),)
        }
    }

    impl<S, I, A1, A2> FetchAll<S, I> for (A1, A2)
    where
        A1: Fetch<S, I>,
        A2: Fetch<S, I>,
    {
        fn fetch_all(node: &Node, engine: OperatorSafeEngineCell) -> Self {
            (A1::fetch(node, engine), A2::fetch(node, engine))
        }
    }

    impl<S, I, A1, A2, A3> FetchAll<S, I> for (A1, A2, A3)
    where
        A1: Fetch<S, I>,
        A2: Fetch<S, I>,
        A3: Fetch<S, I>,
    {
        fn fetch_all(node: &Node, engine: OperatorSafeEngineCell) -> Self {
            (
                A1::fetch(node, engine),
                A2::fetch(node, engine),
                A3::fetch(node, engine),
            )
        }
    }

    impl<S, I, A1, A2, A3, A4> FetchAll<S, I> for (A1, A2, A3, A4)
    where
        A1: Fetch<S, I>,
        A2: Fetch<S, I>,
        A3: Fetch<S, I>,
        A4: Fetch<S, I>,
    {
        fn fetch_all(node: &Node, engine: OperatorSafeEngineCell) -> Self {
            (
                A1::fetch(node, engine),
                A2::fetch(node, engine),
                A3::fetch(node, engine),
                A4::fetch(node, engine),
            )
        }
    }

    impl<S, I, A1, A2, A3, A4, A5> FetchAll<S, I> for (A1, A2, A3, A4, A5)
    where
        A1: Fetch<S, I>,
        A2: Fetch<S, I>,
        A3: Fetch<S, I>,
        A4: Fetch<S, I>,
        A5: Fetch<S, I>,
    {
        fn fetch_all(node: &Node, engine: OperatorSafeEngineCell) -> Self {
            (
                A1::fetch(node, engine),
                A2::fetch(node, engine),
                A3::fetch(node, engine),
                A4::fetch(node, engine),
                A5::fetch(node, engine),
            )
        }
    }

    impl<S, I, A1, A2, A3, A4, A5, A6> FetchAll<S, I> for (A1, A2, A3, A4, A5, A6)
    where
        A1: Fetch<S, I>,
        A2: Fetch<S, I>,
        A3: Fetch<S, I>,
        A4: Fetch<S, I>,
        A5: Fetch<S, I>,
        A6: Fetch<S, I>,
    {
        fn fetch_all(node: &Node, engine: OperatorSafeEngineCell) -> Self {
            (
                A1::fetch(node, engine),
                A2::fetch(node, engine),
                A3::fetch(node, engine),
                A4::fetch(node, engine),
                A5::fetch(node, engine),
                A6::fetch(node, engine),
            )
        }
    }

    impl<S, I, A1, A2, A3, A4, A5, A6, A7> FetchAll<S, I> for (A1, A2, A3, A4, A5, A6, A7)
    where
        A1: Fetch<S, I>,
        A2: Fetch<S, I>,
        A3: Fetch<S, I>,
        A4: Fetch<S, I>,
        A5: Fetch<S, I>,
        A6: Fetch<S, I>,
        A7: Fetch<S, I>,
    {
        fn fetch_all(node: &Node, engine: OperatorSafeEngineCell) -> Self {
            (
                A1::fetch(node, engine),
                A2::fetch(node, engine),
                A3::fetch(node, engine),
                A4::fetch(node, engine),
                A5::fetch(node, engine),
                A6::fetch(node, engine),
                A7::fetch(node, engine),
            )
        }
    }

    impl<S, I, A1, A2, A3, A4, A5, A6, A7, A8> FetchAll<S, I> for (A1, A2, A3, A4, A5, A6, A7, A8)
    where
        A1: Fetch<S, I>,
        A2: Fetch<S, I>,
        A3: Fetch<S, I>,
        A4: Fetch<S, I>,
        A5: Fetch<S, I>,
        A6: Fetch<S, I>,
        A7: Fetch<S, I>,
        A8: Fetch<S, I>,
    {
        fn fetch_all(node: &Node, engine: OperatorSafeEngineCell) -> Self {
            (
                A1::fetch(node, engine),
                A2::fetch(node, engine),
                A3::fetch(node, engine),
                A4::fetch(node, engine),
                A5::fetch(node, engine),
                A6::fetch(node, engine),
                A7::fetch(node, engine),
                A8::fetch(node, engine),
            )
        }
    }

    impl<S, I, A1, A2, A3, A4, A5, A6, A7, A8, A9> FetchAll<S, I>
        for (A1, A2, A3, A4, A5, A6, A7, A8, A9)
    where
        A1: Fetch<S, I>,
        A2: Fetch<S, I>,
        A3: Fetch<S, I>,
        A4: Fetch<S, I>,
        A5: Fetch<S, I>,
        A6: Fetch<S, I>,
        A7: Fetch<S, I>,
        A8: Fetch<S, I>,
        A9: Fetch<S, I>,
    {
        fn fetch_all(node: &Node, engine: OperatorSafeEngineCell) -> Self {
            (
                A1::fetch(node, engine),
                A2::fetch(node, engine),
                A3::fetch(node, engine),
                A4::fetch(node, engine),
                A5::fetch(node, engine),
                A6::fetch(node, engine),
                A7::fetch(node, engine),
                A8::fetch(node, engine),
                A9::fetch(node, engine),
            )
        }
    }

    impl<S, I, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10> FetchAll<S, I>
        for (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10)
    where
        A1: Fetch<S, I>,
        A2: Fetch<S, I>,
        A3: Fetch<S, I>,
        A4: Fetch<S, I>,
        A5: Fetch<S, I>,
        A6: Fetch<S, I>,
        A7: Fetch<S, I>,
        A8: Fetch<S, I>,
        A9: Fetch<S, I>,
        A10: Fetch<S, I>,
    {
        fn fetch_all(node: &Node, engine: OperatorSafeEngineCell) -> Self {
            (
                A1::fetch(node, engine),
                A2::fetch(node, engine),
                A3::fetch(node, engine),
                A4::fetch(node, engine),
                A5::fetch(node, engine),
                A6::fetch(node, engine),
                A7::fetch(node, engine),
                A8::fetch(node, engine),
                A9::fetch(node, engine),
                A10::fetch(node, engine),
            )
        }
    }

    impl<S, I, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11> FetchAll<S, I>
        for (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11)
    where
        A1: Fetch<S, I>,
        A2: Fetch<S, I>,
        A3: Fetch<S, I>,
        A4: Fetch<S, I>,
        A5: Fetch<S, I>,
        A6: Fetch<S, I>,
        A7: Fetch<S, I>,
        A8: Fetch<S, I>,
        A9: Fetch<S, I>,
        A10: Fetch<S, I>,
        A11: Fetch<S, I>,
    {
        fn fetch_all(node: &Node, engine: OperatorSafeEngineCell) -> Self {
            (
                A1::fetch(node, engine),
                A2::fetch(node, engine),
                A3::fetch(node, engine),
                A4::fetch(node, engine),
                A5::fetch(node, engine),
                A6::fetch(node, engine),
                A7::fetch(node, engine),
                A8::fetch(node, engine),
                A9::fetch(node, engine),
                A10::fetch(node, engine),
                A11::fetch(node, engine),
            )
        }
    }

    impl<S, I, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12> FetchAll<S, I>
        for (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12)
    where
        A1: Fetch<S, I>,
        A2: Fetch<S, I>,
        A3: Fetch<S, I>,
        A4: Fetch<S, I>,
        A5: Fetch<S, I>,
        A6: Fetch<S, I>,
        A7: Fetch<S, I>,
        A8: Fetch<S, I>,
        A9: Fetch<S, I>,
        A10: Fetch<S, I>,
        A11: Fetch<S, I>,
        A12: Fetch<S, I>,
    {
        fn fetch_all(node: &Node, engine: OperatorSafeEngineCell) -> Self {
            (
                A1::fetch(node, engine),
                A2::fetch(node, engine),
                A3::fetch(node, engine),
                A4::fetch(node, engine),
                A5::fetch(node, engine),
                A6::fetch(node, engine),
                A7::fetch(node, engine),
                A8::fetch(node, engine),
                A9::fetch(node, engine),
                A10::fetch(node, engine),
                A11::fetch(node, engine),
                A12::fetch(node, engine),
            )
        }
    }

    impl<S, I, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13> FetchAll<S, I>
        for (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13)
    where
        A1: Fetch<S, I>,
        A2: Fetch<S, I>,
        A3: Fetch<S, I>,
        A4: Fetch<S, I>,
        A5: Fetch<S, I>,
        A6: Fetch<S, I>,
        A7: Fetch<S, I>,
        A8: Fetch<S, I>,
        A9: Fetch<S, I>,
        A10: Fetch<S, I>,
        A11: Fetch<S, I>,
        A12: Fetch<S, I>,
        A13: Fetch<S, I>,
    {
        fn fetch_all(node: &Node, engine: OperatorSafeEngineCell) -> Self {
            (
                A1::fetch(node, engine),
                A2::fetch(node, engine),
                A3::fetch(node, engine),
                A4::fetch(node, engine),
                A5::fetch(node, engine),
                A6::fetch(node, engine),
                A7::fetch(node, engine),
                A8::fetch(node, engine),
                A9::fetch(node, engine),
                A10::fetch(node, engine),
                A11::fetch(node, engine),
                A12::fetch(node, engine),
                A13::fetch(node, engine),
            )
        }
    }

    impl<S, I, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14> FetchAll<S, I>
        for (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14)
    where
        A1: Fetch<S, I>,
        A2: Fetch<S, I>,
        A3: Fetch<S, I>,
        A4: Fetch<S, I>,
        A5: Fetch<S, I>,
        A6: Fetch<S, I>,
        A7: Fetch<S, I>,
        A8: Fetch<S, I>,
        A9: Fetch<S, I>,
        A10: Fetch<S, I>,
        A11: Fetch<S, I>,
        A12: Fetch<S, I>,
        A13: Fetch<S, I>,
        A14: Fetch<S, I>,
    {
        fn fetch_all(node: &Node, engine: OperatorSafeEngineCell) -> Self {
            (
                A1::fetch(node, engine),
                A2::fetch(node, engine),
                A3::fetch(node, engine),
                A4::fetch(node, engine),
                A5::fetch(node, engine),
                A6::fetch(node, engine),
                A7::fetch(node, engine),
                A8::fetch(node, engine),
                A9::fetch(node, engine),
                A10::fetch(node, engine),
                A11::fetch(node, engine),
                A12::fetch(node, engine),
                A13::fetch(node, engine),
                A14::fetch(node, engine),
            )
        }
    }

    impl<S, I, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15> FetchAll<S, I>
        for (
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        )
    where
        A1: Fetch<S, I>,
        A2: Fetch<S, I>,
        A3: Fetch<S, I>,
        A4: Fetch<S, I>,
        A5: Fetch<S, I>,
        A6: Fetch<S, I>,
        A7: Fetch<S, I>,
        A8: Fetch<S, I>,
        A9: Fetch<S, I>,
        A10: Fetch<S, I>,
        A11: Fetch<S, I>,
        A12: Fetch<S, I>,
        A13: Fetch<S, I>,
        A14: Fetch<S, I>,
        A15: Fetch<S, I>,
    {
        fn fetch_all(node: &Node, engine: OperatorSafeEngineCell) -> Self {
            (
                A1::fetch(node, engine),
                A2::fetch(node, engine),
                A3::fetch(node, engine),
                A4::fetch(node, engine),
                A5::fetch(node, engine),
                A6::fetch(node, engine),
                A7::fetch(node, engine),
                A8::fetch(node, engine),
                A9::fetch(node, engine),
                A10::fetch(node, engine),
                A11::fetch(node, engine),
                A12::fetch(node, engine),
                A13::fetch(node, engine),
                A14::fetch(node, engine),
                A15::fetch(node, engine),
            )
        }
    }

    pub trait Function<M> {
        type State: InitState;
        type In: InitInputStates;
        type Out;
        type Args: FetchAll<Self::State, Self::In>;
        fn call(&self, args: Self::Args) -> Self::Out;
    }

    impl<F, In, Out, A1, State> Function<(State, Out, In, A1)> for F
    where
        F: Fn(A1) -> Out + 'static,
        In: InitInputStates,
        State: InitState,
        A1: Fetch<State, In>,
        (A1): FetchAll<State, In>,
    {
        type State = State;
        type In = In;
        type Out = Out;
        type Args = (A1);
        fn call(&self, args: Self::Args) -> Self::Out {
            let (A1) = args;
            (*self)(A1)
        }
    }

    impl<F, In, Out, A1, A2, State> Function<(State, Out, In, A1, A2)> for F
    where
        F: Fn(A1, A2) -> Out + 'static,
        In: InitInputStates,
        State: InitState,
        A1: Fetch<State, In>,
        A2: Fetch<State, In>,
        (A1, A2): FetchAll<State, In>,
    {
        type State = State;
        type In = In;
        type Out = Out;
        type Args = (A1, A2);
        fn call(&self, args: Self::Args) -> Self::Out {
            let (A1, A2) = args;
            (*self)(A1, A2)
        }
    }

    impl<F, In, Out, A1, A2, A3, State> Function<(State, Out, In, A1, A2, A3)> for F
    where
        F: Fn(A1, A2, A3) -> Out + 'static,
        In: InitInputStates,
        State: InitState,
        A1: Fetch<State, In>,
        A2: Fetch<State, In>,
        A3: Fetch<State, In>,
        (A1, A2, A3): FetchAll<State, In>,
    {
        type State = State;
        type In = In;
        type Out = Out;
        type Args = (A1, A2, A3);
        fn call(&self, args: Self::Args) -> Self::Out {
            let (A1, A2, A3) = args;
            (*self)(A1, A2, A3)
        }
    }

    impl<F, In, Out, A1, A2, A3, A4, State> Function<(State, Out, In, A1, A2, A3, A4)> for F
    where
        F: Fn(A1, A2, A3, A4) -> Out + 'static,
        In: InitInputStates,
        State: InitState,
        A1: Fetch<State, In>,
        A2: Fetch<State, In>,
        A3: Fetch<State, In>,
        A4: Fetch<State, In>,
        (A1, A2, A3, A4): FetchAll<State, In>,
    {
        type State = State;
        type In = In;
        type Out = Out;
        type Args = (A1, A2, A3, A4);
        fn call(&self, args: Self::Args) -> Self::Out {
            let (A1, A2, A3, A4) = args;
            (*self)(A1, A2, A3, A4)
        }
    }

    impl<F, In, Out, A1, A2, A3, A4, A5, State> Function<(State, Out, In, A1, A2, A3, A4, A5)> for F
    where
        F: Fn(A1, A2, A3, A4, A5) -> Out + 'static,
        In: InitInputStates,
        State: InitState,
        A1: Fetch<State, In>,
        A2: Fetch<State, In>,
        A3: Fetch<State, In>,
        A4: Fetch<State, In>,
        A5: Fetch<State, In>,
        (A1, A2, A3, A4, A5): FetchAll<State, In>,
    {
        type State = State;
        type In = In;
        type Out = Out;
        type Args = (A1, A2, A3, A4, A5);
        fn call(&self, args: Self::Args) -> Self::Out {
            let (A1, A2, A3, A4, A5) = args;
            (*self)(A1, A2, A3, A4, A5)
        }
    }

    #[allow(non_snake_case)]
    impl<F, In, Out, A1, A2, A3, A4, A5, A6, State>
        Function<(State, Out, In, A1, A2, A3, A4, A5, A6)> for F
    where
        F: Fn(A1, A2, A3, A4, A5, A6) -> Out + 'static,
        In: InitInputStates,
        State: InitState,
        A1: Fetch<State, In>,
        A2: Fetch<State, In>,
        A3: Fetch<State, In>,
        A4: Fetch<State, In>,
        A5: Fetch<State, In>,
        A6: Fetch<State, In>,
        (A1, A2, A3, A4, A5, A6): FetchAll<State, In>,
    {
        type State = State;
        type In = In;
        type Out = Out;
        type Args = (A1, A2, A3, A4, A5, A6);
        fn call(&self, args: Self::Args) -> Self::Out {
            let (A1, A2, A3, A4, A5, A6) = args;
            (*self)(A1, A2, A3, A4, A5, A6)
        }
    }

    impl<F, In, Out, A1, A2, A3, A4, A5, A6, A7, State>
        Function<(State, Out, In, A1, A2, A3, A4, A5, A6, A7)> for F
    where
        F: Fn(A1, A2, A3, A4, A5, A6, A7) -> Out + 'static,
        In: InitInputStates,
        State: InitState,
        A1: Fetch<State, In>,
        A2: Fetch<State, In>,
        A3: Fetch<State, In>,
        A4: Fetch<State, In>,
        A5: Fetch<State, In>,
        A6: Fetch<State, In>,
        A7: Fetch<State, In>,
        (A1, A2, A3, A4, A5, A6, A7): FetchAll<State, In>,
    {
        type State = State;
        type In = In;
        type Out = Out;
        type Args = (A1, A2, A3, A4, A5, A6, A7);

        #[allow(non_snake_case)]
        fn call(&self, args: Self::Args) -> Self::Out {
            let (A1, A2, A3, A4, A5, A6, A7) = args;
            (*self)(A1, A2, A3, A4, A5, A6, A7)
        }
    }

    impl<F, In, Out, A1, A2, A3, A4, A5, A6, A7, A8, State>
        Function<(State, Out, In, A1, A2, A3, A4, A5, A6, A7, A8)> for F
    where
        F: Fn(A1, A2, A3, A4, A5, A6, A7, A8) -> Out + 'static,
        In: InitInputStates,
        State: InitState,
        A1: Fetch<State, In>,
        A2: Fetch<State, In>,
        A3: Fetch<State, In>,
        A4: Fetch<State, In>,
        A5: Fetch<State, In>,
        A6: Fetch<State, In>,
        A7: Fetch<State, In>,
        A8: Fetch<State, In>,
        (A1, A2, A3, A4, A5, A6, A7, A8): FetchAll<State, In>,
    {
        type State = State;
        type In = In;
        type Out = Out;
        type Args = (A1, A2, A3, A4, A5, A6, A7, A8);
        fn call(&self, args: Self::Args) -> Self::Out {
            let (A1, A2, A3, A4, A5, A6, A7, A8) = args;
            (*self)(A1, A2, A3, A4, A5, A6, A7, A8)
        }
    }

    impl<F, In, Out, A1, A2, A3, A4, A5, A6, A7, A8, A9, State>
        Function<(State, Out, In, A1, A2, A3, A4, A5, A6, A7, A8, A9)> for F
    where
        F: Fn(A1, A2, A3, A4, A5, A6, A7, A8, A9) -> Out + 'static,
        In: InitInputStates,
        State: InitState,
        A1: Fetch<State, In>,
        A2: Fetch<State, In>,
        A3: Fetch<State, In>,
        A4: Fetch<State, In>,
        A5: Fetch<State, In>,
        A6: Fetch<State, In>,
        A7: Fetch<State, In>,
        A8: Fetch<State, In>,
        A9: Fetch<State, In>,
        (A1, A2, A3, A4, A5, A6, A7, A8, A9): FetchAll<State, In>,
    {
        type State = State;
        type In = In;
        type Out = Out;
        type Args = (A1, A2, A3, A4, A5, A6, A7, A8, A9);
        fn call(&self, args: Self::Args) -> Self::Out {
            let (A1, A2, A3, A4, A5, A6, A7, A8, A9) = args;
            (*self)(A1, A2, A3, A4, A5, A6, A7, A8, A9)
        }
    }

    impl<F, In, Out, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, State>
        Function<(State, Out, In, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10)> for F
    where
        F: Fn(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10) -> Out + 'static,
        In: InitInputStates,
        State: InitState,
        A1: Fetch<State, In>,
        A2: Fetch<State, In>,
        A3: Fetch<State, In>,
        A4: Fetch<State, In>,
        A5: Fetch<State, In>,
        A6: Fetch<State, In>,
        A7: Fetch<State, In>,
        A8: Fetch<State, In>,
        A9: Fetch<State, In>,
        A10: Fetch<State, In>,
        (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10): FetchAll<State, In>,
    {
        type State = State;
        type In = In;
        type Out = Out;
        type Args = (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);
        fn call(&self, args: Self::Args) -> Self::Out {
            let (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10) = args;
            (*self)(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10)
        }
    }

    impl<F, In, Out, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, State>
        Function<(State, Out, In, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11)> for F
    where
        F: Fn(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) -> Out + 'static,
        In: InitInputStates,
        State: InitState,
        A1: Fetch<State, In>,
        A2: Fetch<State, In>,
        A3: Fetch<State, In>,
        A4: Fetch<State, In>,
        A5: Fetch<State, In>,
        A6: Fetch<State, In>,
        A7: Fetch<State, In>,
        A8: Fetch<State, In>,
        A9: Fetch<State, In>,
        A10: Fetch<State, In>,
        A11: Fetch<State, In>,
        (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11): FetchAll<State, In>,
    {
        type State = State;
        type In = In;
        type Out = Out;
        type Args = (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11);
        fn call(&self, args: Self::Args) -> Self::Out {
            let (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11) = args;
            (*self)(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11)
        }
    }

    impl<F, In, Out, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, State>
        Function<(
            State,
            Out,
            In,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
        )> for F
    where
        F: Fn(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12) -> Out + 'static,
        In: InitInputStates,
        State: InitState,
        A1: Fetch<State, In>,
        A2: Fetch<State, In>,
        A3: Fetch<State, In>,
        A4: Fetch<State, In>,
        A5: Fetch<State, In>,
        A6: Fetch<State, In>,
        A7: Fetch<State, In>,
        A8: Fetch<State, In>,
        A9: Fetch<State, In>,
        A10: Fetch<State, In>,
        A11: Fetch<State, In>,
        A12: Fetch<State, In>,
        (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12): FetchAll<State, In>,
    {
        type State = State;
        type In = In;
        type Out = Out;
        type Args = (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12);
        fn call(&self, args: Self::Args) -> Self::Out {
            let (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12) = args;
            (*self)(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12)
        }
    }

    impl<F, In, Out, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, State>
        Function<(
            State,
            Out,
            In,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
        )> for F
    where
        F: Fn(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13) -> Out + 'static,
        In: InitInputStates,
        State: InitState,
        A1: Fetch<State, In>,
        A2: Fetch<State, In>,
        A3: Fetch<State, In>,
        A4: Fetch<State, In>,
        A5: Fetch<State, In>,
        A6: Fetch<State, In>,
        A7: Fetch<State, In>,
        A8: Fetch<State, In>,
        A9: Fetch<State, In>,
        A10: Fetch<State, In>,
        A11: Fetch<State, In>,
        A12: Fetch<State, In>,
        A13: Fetch<State, In>,
        (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13): FetchAll<State, In>,
    {
        type State = State;
        type In = In;
        type Out = Out;
        type Args = (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13);
        fn call(&self, args: Self::Args) -> Self::Out {
            let (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13) = args;
            (*self)(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13)
        }
    }

    impl<F, In, Out, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, State>
        Function<(
            State,
            Out,
            In,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
        )> for F
    where
        F: Fn(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14) -> Out + 'static,
        In: InitInputStates,
        State: InitState,
        A1: Fetch<State, In>,
        A2: Fetch<State, In>,
        A3: Fetch<State, In>,
        A4: Fetch<State, In>,
        A5: Fetch<State, In>,
        A6: Fetch<State, In>,
        A7: Fetch<State, In>,
        A8: Fetch<State, In>,
        A9: Fetch<State, In>,
        A10: Fetch<State, In>,
        A11: Fetch<State, In>,
        A12: Fetch<State, In>,
        A13: Fetch<State, In>,
        A14: Fetch<State, In>,
        (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14): FetchAll<State, In>,
    {
        type State = State;
        type In = In;
        type Out = Out;
        type Args = (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14);
        fn call(&self, args: Self::Args) -> Self::Out {
            let (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14) = args;
            (*self)(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14)
        }
    }

    impl<F, In, Out, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, State>
        Function<(
            State,
            Out,
            In,
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        )> for F
    where
        F: Fn(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15) -> Out + 'static,
        In: InitInputStates,
        State: InitState,
        A1: Fetch<State, In>,
        A2: Fetch<State, In>,
        A3: Fetch<State, In>,
        A4: Fetch<State, In>,
        A5: Fetch<State, In>,
        A6: Fetch<State, In>,
        A7: Fetch<State, In>,
        A8: Fetch<State, In>,
        A9: Fetch<State, In>,
        A10: Fetch<State, In>,
        A11: Fetch<State, In>,
        A12: Fetch<State, In>,
        A13: Fetch<State, In>,
        A14: Fetch<State, In>,
        A15: Fetch<State, In>,
        (
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        ): FetchAll<State, In>,
    {
        type State = State;
        type In = In;
        type Out = Out;
        type Args = (
            A1,
            A2,
            A3,
            A4,
            A5,
            A6,
            A7,
            A8,
            A9,
            A10,
            A11,
            A12,
            A13,
            A14,
            A15,
        );
        fn call(&self, args: Self::Args) -> Self::Out {
            let (A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15) = args;
            (*self)(
                A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15,
            )
        }
    }

    pub struct FunctionDetails<M, F>
    where
        F: Function<M>,
    {
        function: F,
        marker: PhantomData<fn() -> M>,
    }

    impl<M, F> FunctionDetails<M, F>
    where
        F: Function<M> + 'static,
    {
        pub fn new(function: F) -> Self {
            Self {
                function,
                marker: Default::default(),
            }
        }
    }

    impl<M, F> Operator for FunctionDetails<M, F>
    where
        F: Function<M> + 'static,
    {
        fn evaluate(&self, node: &mut Node, engine: OperatorSafeEngineCell) {
            let args = F::Args::fetch_all(node, engine);
            _ = self.function.call(args);
        }
        fn create_node_storage(&self) -> NodeOperatorStorage {
            NodeOperatorStorage {
                operator_type_id: TypeId::of::<F>(),
                state: F::State::init_state(),
                inputs: F::In::init_input_states(),
            }
        }
    }

    /// A function that is being used as an operator
    pub trait IntoOperator<M> {
        type Operator: Operator;
        fn into_operator(self) -> Self::Operator;
    }

    impl<M, F> IntoOperator<M> for F
    where
        F: Function<M> + Sized + 'static,
    {
        type Operator = FunctionDetails<M, F>;
        fn into_operator(self) -> Self::Operator {
            FunctionDetails::new(self)
        }
    }
}
