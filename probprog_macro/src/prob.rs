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

    sample_wrap(block, &func.sig.ident);

    closure_wrap(block, &original_return_type);

    // bump_returns(block);

    // println!("{}", func.clone().into_token_stream().to_string());

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
fn closure_wrap(block: &mut Block, original_return_type: &Type) {
    *block = parse2(quote! {
        {
            move |__trace: &mut ::probprog::trace::Trace| -> ::probprog::distribution::Sample<(#original_return_type)> {
            #block
            }
        }
    }).unwrap()
}

fn sample_wrap(block: &mut Block, funcname: &Ident) {
    let funcname = funcname.to_string();
    *block = parse2(quote! {
        {
            let __trace = __trace.descend(::probprog::trace::TraceDirectory::Function(#funcname.to_string()));
            let mut __log_likelihood = 0.;
            let value = (|| {
                #block
            })();
            ::probprog::distribution::Sample {
                value,
                log_likelihood: __log_likelihood,
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

fn tack_onto_loop_body(input: &Block) -> Block {
    // We should be able to just unwrap here without a chance for error.
    parse2(quote! {
        {
            let __trace = __trace.descend(::probprog::trace::TraceDirectory::Loop(__loop_counter));
            let value = {
                #input
            };
            __loop_counter += 1;
            value
        }
    })
    .unwrap()
}

fn tack_onto_loop_expr(input: TokenStream) -> TokenStream {
    quote! {
        {
            let mut __loop_counter: usize = 0;
            #input
        }
    }
}

// fn bump_returns(mut input: Block) -> Block {
//     BumpReturns.visit_block_mut(&mut input);
//     input
// }

// struct BumpReturns;

// impl VisitMut for BumpReturns {
//     fn visit_expr_return_mut(&mut self, i: &mut ExprReturn) {
//         let new_expr = if let Some(e) = &i.expr {
//             quote! {
//                 Ok(#e)
//             }
//         } else {
//             quote! {
//                 Ok(())
//             }
//         };

//         i.expr = Some(parse2(new_expr).unwrap());
//     }
// }
