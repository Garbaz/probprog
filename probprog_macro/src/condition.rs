use proc_macro as pm;
use quote::quote;
use syn::{parse_macro_input, Expr};

pub fn condition(input: pm::TokenStream) -> pm::TokenStream {
    let expr = parse_macro_input!(input as Expr);

    quote! {
        ::probprog::__inject::condition(
            &mut __total_log_probability,
            (#expr),
        )
    }
    .into()
}
