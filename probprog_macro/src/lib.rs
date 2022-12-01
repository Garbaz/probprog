extern crate proc_macro;

use probprog_core::*;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    self,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Expr, Ident, ItemFn, Stmt, Token,
};


#[proc_macro_attribute]
pub fn prob(attrs: TokenStream, input: TokenStream) -> TokenStream {
    // let attrs = Punctuated::<TokenStream, Token![,]>::parse_terminated(attrs);
    // println!("{:#?}", attrs);
    let ast = input.clone();
    let ast = parse_macro_input!(ast as ItemFn);
    // println!("{:#?}", ast);
    // ast.to_token_stream().into()
    input
}
