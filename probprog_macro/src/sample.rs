extern crate proc_macro;
use proc_macro as pm;
use quote::quote;
use syn::{parse_macro_input, Expr};

pub fn sample(input: pm::TokenStream) -> pm::TokenStream {
    let expr = parse_macro_input!(input as Expr);

    quote! {
        ::probprog::__inject::sample(
            &mut __old_traces,
            __new_traces,
            &mut __total_log_probability,
            (#expr),
        )
    }
    .into()
}
