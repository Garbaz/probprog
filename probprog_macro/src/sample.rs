extern crate proc_macro;
use proc_macro as pm;
use quote::quote;
use syn::{parse_macro_input, Expr};

pub fn sample(input: pm::TokenStream) -> pm::TokenStream {
    let expr = parse_macro_input!(input as Expr);

    quote! {
        probprog::__internal::sample(
            __trace,
            &mut __log_probability,
            #expr,
        )
    }
    .into()
}
