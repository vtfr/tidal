use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::token::{For, Impl};

pub struct EvaluatorAttributes {
    pub implement_struct: bool,
    pub ident: Ident,
    pub operator_name: String,
}

impl Parse for EvaluatorAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let not_implement_struct: Option<Impl> = input.parse()?;
        let ident: Ident = input.parse()?;
        let _: For = input.parse()?;
        let operator_ident: Ident = input.parse()?;

        Ok(Self {
            implement_struct: not_implement_struct.is_none(),
            ident,
            operator_name: operator_ident.to_string(),
        })
    }
}
