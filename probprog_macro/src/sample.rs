extern crate proc_macro;
use proc_macro as pm;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

pub fn sample(input: pm::TokenStream) -> pm::TokenStream {
    let input = parse_macro_input!(input as Expr);

    let output = sample_expr(input);

    output.into()
}

pub fn s(input: pm::TokenStream) -> pm::TokenStream {
    let input = parse_macro_input!(input as Expr);

    let output = sample_expr(input);

    output.into()
}

fn sample_expr(expr: Expr) -> TokenStream {
    quote! {
        (::probprog::__internal::probfunc::traced_sample(
            &mut (#expr),
            &mut __probprog_tracing_path,
            __probprog_tracing_data,
        ))
    }
}
