use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::LitFloat;

#[derive(Debug, Clone)]
pub struct ScalarDefaultValue(LitFloat);

impl Parse for ScalarDefaultValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl ToTokens for ScalarDefaultValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let v = &self.0;

        tokens.extend(quote! { #v });
    }
}

#[derive(Debug, Clone)]
pub struct VectorDefaultValue(LitFloat, LitFloat, LitFloat);

impl Parse for VectorDefaultValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let x: LitFloat = input.parse()?;
        let _: Comma = input.parse()?;
        let y: LitFloat = input.parse()?;
        let _: Comma = input.parse()?;
        let z: LitFloat = input.parse()?;

        Ok(Self(x, y, z))
    }
}

impl ToTokens for VectorDefaultValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let x = &self.0;
        let y = &self.1;
        let z = &self.2;

        tokens.extend(quote! {
            cgmath::Vector3::new(#x, #y, #z)
        })
    }
}

#[derive(Debug, Clone, Default)]
pub enum DefaultValue {
    Scalar(ScalarDefaultValue),
    Vector(VectorDefaultValue),

    #[default]
    None,
}

impl ToTokens for DefaultValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            DefaultValue::Scalar(s) => tokens.extend(quote! { Some(Constant::Scalar(#s)) }),
            DefaultValue::Vector(v) => tokens.extend(quote! { Some(Constant::Vector(#v)) }),
            DefaultValue::None => tokens.extend(quote! { None }),
        }
    }
}
