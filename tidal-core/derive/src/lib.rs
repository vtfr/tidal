#![feature(proc_macro_quote)]

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Error, ItemFn};

use crate::evaluator_attributes::EvaluatorAttributes;
use crate::helper::ErrorAccumulator;

mod evaluator_attributes;
mod evaluator_data_type;
mod evaluator_impl;
mod evaluator_input;
mod evaluator_input_default_value;
mod evaluator_output;
mod helper;

#[proc_macro_attribute]
pub fn evaluator(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = parse_macro_input!(args as EvaluatorAttributes);
    let item = parse_macro_input!(item as ItemFn);

    evaluator_impl::derive_evaluator_impl(args, item)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
