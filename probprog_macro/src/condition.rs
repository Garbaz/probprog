
use proc_macro as pm;
use proc_macro2::TokenStream;
use quote::quote;

pub fn condition(input : pm::TokenStream) -> pm::TokenStream {
    let input : TokenStream = input.into();
    let input_str = input.to_string();
    quote! {
        if ! (#input) {
            return ::std::result::Result::Err(::probprog::__internal::probfunc::ConditionError::new(#input_str));
        } 
    }.into()
}