extern crate proc_macro;
use proc_macro as pm;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse2, parse_macro_input,
    visit_mut::{self, VisitMut},
    Block, Expr, ItemFn, ReturnType, Token, Type,
};

pub fn prob(
    _attrs: pm::TokenStream,
    input: pm::TokenStream,
) -> pm::TokenStream {
    let mut func = parse_macro_input!(input as ItemFn);

    let original_return_type = return_type(&mut func.sig.output);

    let block = func.block.as_mut();

    loop_descend(block);

    inner_closure(block, &original_return_type);

    outer_closure(block, &func.sig.ident, &original_return_type);

    func.into_token_stream().into()
}

/// Construct the new return type of the function. If it before was `f64`, it
/// now is `impl FnProb<f64>`.
fn return_type(func_output: &mut ReturnType) -> Type {
    let mut rarrow = Token![->](Span::call_site());
    let orig_func_output = match func_output.clone() {
        ReturnType::Default => parse2(quote! {()}).unwrap(),
        ReturnType::Type(a, t) => {
            rarrow = a;
            t
        }
    };

    let new_return_type = parse2(quote! {
        impl ::probprog::distribution::FnProb<#orig_func_output>
    })
    .unwrap();

    *func_output = ReturnType::Type(rarrow, Box::new(new_return_type));

    *orig_func_output
}

/// Construct the new function body. If it before was `{17.29}`, it now is
/// `{move |trace : &mut Trace| -> Sample<f64> {17.29})}`.
fn outer_closure(
    block: &mut Block,
    funcname: &Ident,
    original_return_type: &Type,
) {
    let funcname = funcname.to_string();
    *block = parse2(quote! {
        {
            const __FUNCTION_NAME: &str = #funcname;
            move |__old_trace: ::probprog::trace::Trace| -> ::probprog::distribution::TracedSample<#original_return_type>
                #block
        }
    }).unwrap()
}

fn inner_closure(block: &mut Block, original_return_type: &Type) {
    *block = parse2(quote! {
        {
            let mut __new_trace = ::probprog::trace::Trace::Function {
                name: __FUNCTION_NAME.to_string(),
                subtraces: ::probprog::trace::Traces::new(),
            };

            let mut __total_log_probability = 0.;

            let mut __old_traces =
                __old_trace.function_subtraces(__FUNCTION_NAME);
            let __new_traces = __new_trace.subtraces().unwrap();

            let return_value = (|| -> #original_return_type
                #block
            )();

            ::probprog::distribution::TracedSample {
                sample: ::probprog::distribution::Sample {
                    value: return_value,
                    log_probability: __total_log_probability,
                },
                trace: __new_trace,
            }
        }
    })
    .unwrap()
}

fn loop_descend(input: &mut Block) {
    TackOnLoopTracing.visit_block_mut(input);
}

struct TackOnLoopTracing;

impl VisitMut for TackOnLoopTracing {
    fn visit_expr_mut(&mut self, i: &mut Expr) {
        match i {
            Expr::ForLoop(l) => {
                self.visit_block_mut(&mut l.body);
                l.body = tack_onto_loop_body(&l.body);
                *i =
                    parse2(tack_onto_loop_expr(i.into_token_stream())).unwrap();
            }
            Expr::Loop(l) => {
                self.visit_block_mut(&mut l.body);
                l.body = tack_onto_loop_body(&l.body);
                *i =
                    parse2(tack_onto_loop_expr(i.into_token_stream())).unwrap();
            }
            Expr::While(l) => {
                self.visit_block_mut(&mut l.body);
                l.body = tack_onto_loop_body(&l.body);
                *i =
                    parse2(tack_onto_loop_expr(i.into_token_stream())).unwrap();
            }
            _ => visit_mut::visit_expr_mut(self, i),
        };
    }
}

fn tack_onto_loop_body(block: &Block) -> Block {
    // We should be able to just unwrap here without a chance for error.
    parse2(quote! {
        {
            let (mut __old_traces, __new_traces) =
                        ::probprog::__inject::loop_descend(
                            &mut __old_traces,
                            __new_traces,
                            __loop_counter,
                        );
            let return_value = #block;
            __loop_counter += 1;
            return_value
        }
    })
    .unwrap()
}

fn tack_onto_loop_expr(loop_expr: TokenStream) -> TokenStream {
    quote! {
        {
            let mut __loop_counter = 0;
            #loop_expr
        }
    }
}
