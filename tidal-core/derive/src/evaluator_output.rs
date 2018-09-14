use proc_macro2::{Ident, Span};
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{
    Attribute, Error, Expr, FnArg, Lit, LitStr, MetaNameValue, Pat, PatIdent, PatType, ReturnType,
    Type, TypePath,
};

use crate::evaluator_input::DataType;
use crate::helper::ErrorAccumulator;
use crate::{evaluator_impl, helper};

#[derive(Debug)]
pub struct Output {
    pub attributes: OutputAttribute,
    pub data_type: DataType,
}

#[derive(Debug)]
pub struct OutputAttribute {
    pub name: LitStr,
}

impl Parse for OutputAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let kv = Punctuated::<MetaNameValue, Comma>::parse_terminated(input)?;

        let mut errors = ErrorAccumulator::default();
        let mut name = None;

        for meta in kv.into_iter() {
            if meta.path.is_ident("name") {
                match meta.value {
                    Expr::Lit(expr) => match expr.lit {
                        Lit::Str(expr) => name = Some(expr),
                        _ => errors.add(Error::new(expr.span(), "unexpected value")),
                    },
                    _ => errors.add(Error::new(meta.span(), "unexpected value")),
                }
            }
        }

        errors.accumulate()?;

        let Some(name) = name else {
            return Err(Error::new(input.span(), "missing output name"));
        };

        Ok(Self { name })
    }
}

#[derive(Debug)]
pub enum ReturnStyle {
    Tuple,
    Single,
}

#[derive(Debug)]
pub struct Outputs {
    pub items: Vec<Output>,
    pub return_style: ReturnStyle,
}

impl TryFrom<(&[Attribute], &ReturnType)> for Outputs {
    type Error = Error;

    fn try_from(
        (attributes, return_type): (&[Attribute], &ReturnType),
    ) -> Result<Self, Self::Error> {
        macro_rules! unsupported_return_type {
            ($span:expr) => {
                return Err(Error::new($span, "unsupported return type"))
            };
        }

        let mut output_attributes = vec![];

        for attr in attributes.iter() {
            if attr.path().is_ident("output") {
                let meta = attr.meta.require_list()?;

                let tokens = meta.tokens.clone();
                let output_attribute = OutputAttribute::parse.parse2(tokens)?;

                output_attributes.push(output_attribute);
            }
        }

        let (data_types, return_style) = match &return_type {
            ReturnType::Default => (vec![], ReturnStyle::Tuple),
            ReturnType::Type(_, ty) => match ty.as_ref() {
                Type::Paren(ty) => match &*ty.elem {
                    Type::Path(ty) => (vec![DataType::try_from(ty)?], ReturnStyle::Tuple),
                    _ => unsupported_return_type!(ty.span()),
                },
                Type::Path(ty) => (vec![ty.try_into()?], ReturnStyle::Single),
                Type::Tuple(ty) => {
                    let mut data_types = vec![];

                    for elem in ty.elems.iter() {
                        match elem {
                            Type::Path(ty) => {
                                data_types.push(ty.try_into()?);
                            }
                            _ => unsupported_return_type!(elem.span()),
                        }
                    }

                    (data_types, ReturnStyle::Tuple)
                }
                _ => unsupported_return_type!(ty.span()),
            },
        };

        if data_types.len() != output_attributes.len() {
            return Err(Error::new(
                Span::call_site(),
                "output attributes must match output values",
            ));
        }

        let items = data_types
            .into_iter()
            .zip(output_attributes)
            .map(|(data_type, attributes)| Output {
                attributes,
                data_type,
            })
            .collect();

        Ok(Self {
            items,
            return_style,
        })
    }
}
