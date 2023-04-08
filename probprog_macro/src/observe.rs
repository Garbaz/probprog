extern crate proc_macro;
use proc_macro as pm;
use quote::quote;
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, Error, Expr, Token,
};

struct ObserveArgs(Expr, Expr);

impl Parse for ObserveArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut exprs = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
        if let (Some(v), Some(d)) = (exprs.pop(), exprs.pop()) {
            Ok(Self(d.into_value(), v.into_value()))
        } else {
            Err(Error::new(
                input.span(),
                "expected `distribution, value`",
            ))
        }
    }
}

pub fn observe(input: pm::TokenStream) -> pm::TokenStream {
    let ObserveArgs(expr_distribution, expr_value) =
        parse_macro_input!(input as ObserveArgs);
    
    quote! {
        probprog::__internal::observe(
            &mut __log_likelihood,
            #expr_distribution,
            &(#expr_value),
        )
    }
    .into()
}
