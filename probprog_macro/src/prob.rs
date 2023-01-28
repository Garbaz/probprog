extern crate proc_macro;
use proc_macro as pm;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse2, parse_macro_input,
    visit_mut::{self, VisitMut},
    Block, Expr, ItemFn, ReturnType, Token,
};

pub fn prob(
    _attrs: pm::TokenStream,
    input: pm::TokenStream,
) -> pm::TokenStream {
    let mut func = parse_macro_input!(input as ItemFn);

    func.sig.output = probfunc_return_type(func.sig.output);

    let block = *func.block;

    let block = add_func_tracing(block, &func.sig.ident);

    let block = add_loop_tracing(block);

    let block = probfunc_block(block);

    func.block = Box::new(block);

    func.into_token_stream().into()
}

/// Construct the new return type of the function. If it before was `f64`, it
/// now is `ProbFunc<f64, impl Fn(&mut TracingPathRec, &mut TracingData) -> f64>`.
fn probfunc_return_type(input: ReturnType) -> ReturnType {
    let mut rarrow = Token![->](Span::call_site());
    let orig_func_output = match input {
        ReturnType::Default => quote! {()},
        ReturnType::Type(a, t) => {
            rarrow = a;
            t.into_token_stream()
        }
    };

    let new_func_output = quote! {
        ::probprog::__internal::probfunc::ProbFunc<(#orig_func_output),
            impl Fn(&mut ::probprog::__internal::trace::TracingPathRec,
                    &mut ::probprog::__internal::trace::TracingData) -> (#orig_func_output)>
    }.into();

    // We should be able to just unwrap here without a chance for error.
    let new_func_output = parse2(new_func_output).unwrap();

    ReturnType::Type(rarrow, Box::new(new_func_output))
}

/// Construct the new function body. If it before was `{17.29}`, it now is
/// `{ProbFunc::new(move |...| {17.29})}`.
fn probfunc_block(input: Block) -> Block {
    let old_func_block = input.into_token_stream();
    let new_func_block = quote! {
        {
            ::probprog::__internal::probfunc::ProbFunc::new(
                move | __probprog_tracing_path: &mut ::probprog::__internal::trace::TracingPathRec,
                       __probprog_tracing_data: &mut ::probprog::__internal::trace::TracingData |
                    #old_func_block
            )
        }
    }.into();

    // We should be able to just unwrap here without a chance for error.
    let new_func_block = parse2(new_func_block).unwrap();
    new_func_block
}

fn add_func_tracing(input: Block, funcname: &Ident) -> Block {
    let funcname = &funcname.to_string()[..];
    parse2(quote! {
        {   let mut __probprog_tracing_path =
                __probprog_tracing_path.descend_function(#funcname);
            #input
        }
    })
    .unwrap()
}

fn add_loop_tracing(mut input: Block) -> Block {
    TackOnLoopTracing.visit_block_mut(&mut input);
    input
}

struct TackOnLoopTracing;

impl VisitMut for TackOnLoopTracing {
    fn visit_expr_mut(&mut self, i: &mut Expr) {
        match i {
            Expr::ForLoop(l) => {
                self.visit_block_mut(&mut l.body);
                l.body = tack_on_increment_loop(&l.body);
                // We should be able to just unwrap here without a chance for error.
                *i = parse2(tack_on_descend_loop(i.into_token_stream()))
                    .unwrap();
            }
            Expr::Loop(l) => {
                self.visit_block_mut(&mut l.body);
                l.body = tack_on_increment_loop(&l.body);
                // We should be able to just unwrap here without a chance for error.
                *i = parse2(tack_on_descend_loop(i.into_token_stream()))
                    .unwrap();
            }
            Expr::While(l) => {
                self.visit_block_mut(&mut l.body);
                l.body = tack_on_increment_loop(&l.body);
                // We should be able to just unwrap here without a chance for error.
                *i = parse2(tack_on_descend_loop(i.into_token_stream()))
                    .unwrap();
            }
            _ => visit_mut::visit_expr_mut(self, i),
        };
    }
}

fn tack_on_descend_loop(input: TokenStream) -> TokenStream {
    quote! {
        {
            let mut __probprog_tracing_path =
                __probprog_tracing_path.descend_loop();

            #input
        }
    }
}

fn tack_on_increment_loop(input: &Block) -> Block {
    // We should be able to just unwrap here without a chance for error.
    parse2(quote! {
        {
            let __probprog_loop_result = #input;
            __probprog_tracing_path.increment_loop();
            __probprog_loop_result
        }
    })
    .unwrap()
}
