use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, Parser};
use syn::spanned::Spanned;
use syn::{Error, FnArg, Pat, PatIdent, PatType, Type, TypePath};

use crate::evaluator_input_default_value::{DefaultValue, ScalarDefaultValue, VectorDefaultValue};
use crate::{evaluator_impl, helper};

pub enum Argument {
    State,
    Context,
    Input(Input),
}

impl TryFrom<&FnArg> for Argument {
    type Error = Error;

    fn try_from(value: &FnArg) -> Result<Self, Self::Error> {
        match value {
            FnArg::Receiver(receiver) => {
                Err(Error::new(receiver.span(), "receiver input not supported"))
            }
            FnArg::Typed(pat) => {
                if helper::has_attribute(&pat.attrs, "state") {
                    Ok(Argument::State)
                } else if helper::has_attribute(&pat.attrs, "context") {
                    Ok(Argument::Context)
                } else {
                    Ok(Argument::Input(pat.try_into()?))
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DataType {
    Scalar,
    Vector,
    Mesh,
    Command,
}

impl ToTokens for DataType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            DataType::Scalar => tokens.extend(quote! { DataType::Scalar }),
            DataType::Vector => tokens.extend(quote! { DataType::Vector }),
            DataType::Mesh => tokens.extend(quote! { DataType::Mesh }),
            DataType::Command => tokens.extend(quote! { DataType::Command }),
        }
    }
}

#[derive(Debug)]
pub struct Input {
    pub ident: Ident,
    pub multiple: bool,
    pub data_type: DataType,
    pub default_value: DefaultValue,
}

impl TryFrom<&PatType> for Input {
    type Error = Error;

    fn try_from(input: &PatType) -> Result<Self, Self::Error> {
        let Pat::Ident(PatIdent { ident, .. }) = &*input.pat else {
            return Err(Error::new(input.span(), "not supported"));
        };

        let Type::Path(ty) = &*input.ty else {
            return Err(Error::new(input.span(), "not supported"));
        };

        let (ty, multiple) = if helper::path_ends_with(&ty.path, "Multiple") {
            let Some(ty) = helper::get_first_generic_argument(&ty.path) else {
                return Err(Error::new(
                    input.span(),
                    "not supported Multiple call without arguments",
                ));
            };

            (ty, true)
        } else {
            (ty, false)
        };

        let data_type = ty.try_into()?;

        let default_value = if let Some(attr) =
            input.attrs.iter().find(|p| p.path().is_ident("default"))
        {
            if multiple {
                return Err(Error::new(
                    input.span(),
                    "default value for multiple input not supported",
                ));
            }

            let meta = attr.meta.require_list()?;
            let tokens = meta.tokens.clone();

            match data_type {
                DataType::Scalar => DefaultValue::Scalar(ScalarDefaultValue::parse.parse2(tokens)?),
                DataType::Vector => DefaultValue::Vector(VectorDefaultValue::parse.parse2(tokens)?),
                DataType::Mesh => {
                    return Err(Error::new(
                        input.span(),
                        "default value for mesh not supported",
                    ));
                }
                DataType::Command => {
                    return Err(Error::new(
                        input.span(),
                        "default value for command not supported",
                    ));
                }
            }
        } else {
            DefaultValue::None
        };

        match data_type {
            DataType::Scalar => {}
            DataType::Vector => {}
            DataType::Mesh => {}
            DataType::Command => {}
        }

        let ident = ident.clone();
        Ok(Self {
            ident,
            multiple,
            data_type,
            default_value,
        })
    }
}
