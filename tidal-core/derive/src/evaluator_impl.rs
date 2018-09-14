use convert_case::{Case, Casing};
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::{For, Impl};
use syn::{
    Attribute, Data, Error, Field, FnArg, ItemFn, LitInt, LitStr, Pat, PatIdent, PatType,
    ReturnType, Type, TypePath,
};

use crate::evaluator_attributes::EvaluatorAttributes;
use crate::evaluator_input::{Argument, DataType, Input};
use crate::evaluator_input_default_value::DefaultValue;
use crate::evaluator_output::{Outputs, ReturnStyle};
use crate::helper;
use crate::helper::ErrorAccumulator;

pub fn derive_evaluator_impl(
    args: EvaluatorAttributes,
    item: ItemFn,
) -> Result<TokenStream, Error> {
    let mut arguments = vec![];
    let mut errors = ErrorAccumulator::default();

    for input in item.sig.inputs.iter() {
        match Argument::try_from(input) {
            Ok(argument) => {
                arguments.push(argument);
            }
            Err(error) => {
                errors.add(error);
            }
        }
    }

    errors.accumulate()?;

    let outputs = Outputs::try_from((&*item.attrs, &item.sig.output))?;

    let (inputs, input_ident, call_site_argument) = {
        let mut input_counter = 0usize;
        let mut inputs = vec![];
        let mut input_idents = vec![];
        let mut call_site_arguments = vec![];

        for argument in arguments.iter() {
            match argument {
                Argument::State => call_site_arguments.push(quote! { self }),
                Argument::Context => call_site_arguments.push(quote! { ctx }),
                Argument::Input(input) => {
                    inputs.push(input);

                    let ident = format_ident!("i{}", input_counter);
                    input_counter += 1;

                    call_site_arguments.push(ident.clone().to_token_stream());
                    input_idents.push(ident);
                }
            }
        }

        (inputs, input_idents, call_site_arguments)
    };

    let input_name: Vec<String> = inputs
        .iter()
        .map(|i| i.ident.to_string().to_case(Case::UpperCamel))
        .collect::<Vec<_>>();
    let input_data_type: Vec<DataType> = inputs.iter().map(|i| i.data_type).collect();
    let input_default_value: Vec<&DefaultValue> = inputs.iter().map(|i| &i.default_value).collect();

    let input_call = inputs.iter().enumerate().map(|(port, i)| {
        if i.multiple {
            quote! { ctx.evaluate_input_multiple(#port)?.try_into()? }
        } else {
            quote! { ctx.evaluate_input(#port)?.try_into()? }
        }
    });

    let output_name: Vec<&LitStr> = outputs.items.iter().map(|o| &o.attributes.name).collect();
    let output_data_type: Vec<&DataType> = outputs.items.iter().map(|o| &o.data_type).collect();
    let output_ident: Vec<Ident> = (0..outputs.items.len())
        .map(|i| format_ident!("o{}", i))
        .collect();
    let output_port = (0..outputs.items.len());

    let evaluator_ident = &args.ident;

    // Build impl
    let fallible = helper::has_attribute(&item.attrs, "fallible");

    let lhs = match outputs.return_style {
        ReturnStyle::Tuple => {
            quote! {
                let (#(#output_ident),*)
            }
        }
        ReturnStyle::Single => {
            let output_ident = output_ident.first().unwrap();
            quote! {
                let #output_ident
            }
        }
    };

    let fn_ident = &item.sig.ident;
    let rhs = if fallible {
        quote! {
            #fn_ident(#(#call_site_argument),*)?
        }
    } else {
        quote! {
            #fn_ident(#(#call_site_argument),*)
        }
    };

    let operator_name = args.operator_name;

    // Build
    let mut tokens = TokenStream::new();

    // Cleanup function
    let mut item = item.clone();
    item.attrs.retain(helper::retain_attributes);
    item.sig.inputs.iter_mut().for_each(|input| match input {
        FnArg::Receiver(_) => unreachable!(),
        FnArg::Typed(ty) => ty.attrs.retain(helper::retain_attributes),
    });

    // Build fn
    tokens.extend(quote! { #item });

    // Build struct
    if args.implement_struct {
        tokens.extend(quote! {
            #[automatically_derived]
            #[derive(Debug, Copy, Clone, Default)]
            pub struct #evaluator_ident;
        });
    }

    // Build struct impl
    tokens.extend(quote! {
        const _: () = {
            use crate::interpreter::*;
            use crate::operator::*;
            use crate::graph::*;

            #item

            #[automatically_derived]
            impl Evaluate for #evaluator_ident {
                fn evaluate(&mut self, ctx: &mut EvaluateContext) -> Result<(), EvaluateError> {
                    #(
                        let #input_ident = #input_call;
                    )*

                    #lhs = #rhs;

                    #(
                        ctx.write_output(#output_port, #output_ident.into());
                    )*

                    Ok(())
                }
            };

            const _: () = {
                fn create_operator() -> Box<dyn Evaluate> {
                    Box::new(#evaluator_ident::default())
                }

                fn create_metadata() -> Metadata {
                    Metadata {
                        name: #operator_name,
                        description: None,
                        inputs: vec![
                            #(
                               InputMetadata {
                                    name: #input_name,
                                    data_type: #input_data_type,
                                    required: false,
                                    multiple: false,
                                    default: #input_default_value,
                                }
                            ),*
                        ],
                        outputs: vec![
                            #(
                               OutputMetadata {
                                    name: #output_name,
                                    data_type: #output_data_type,
                                }
                            ),*
                        ]
                    }
                }

                inventory::submit!(EvaluatorRegistryNode{
                    operator: #operator_name,
                    create_operator
                });

                inventory::submit!(OperatorMetadataRegistryNode{
                    operator: #operator_name,
                    create_metadata
                });
            };
        };
    });

    Ok(tokens)
}
