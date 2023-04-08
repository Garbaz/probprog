extern crate proc_macro;
use proc_macro as pm;

mod condition;
mod observe;
mod prob;
mod sample;

#[proc_macro_attribute]
pub fn prob(attrs: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    prob::prob(attrs, input)
}

#[proc_macro]
pub fn sample(input: pm::TokenStream) -> pm::TokenStream {
    sample::sample(input)
}

#[proc_macro]
pub fn s(input: pm::TokenStream) -> pm::TokenStream {
    sample::sample(input)
}

#[proc_macro]
pub fn observe(input: pm::TokenStream) -> pm::TokenStream {
    observe::observe(input)
}

#[proc_macro]
pub fn o(input: pm::TokenStream) -> pm::TokenStream {
    observe::observe(input)
}

#[proc_macro]
pub fn condition(input: pm::TokenStream) -> pm::TokenStream {
    condition::condition(input)
}

// #[proc_macro]
// pub fn cond(input: pm::TokenStream) -> pm::TokenStream {
//     condition::condition(input)
// }

#[proc_macro]
pub fn c(input: pm::TokenStream) -> pm::TokenStream {
    condition::condition(input)
}

// mod prob;
// mod sample;

// use proc_macro as pm;
// use proc_macro2::{Group, Punct, Spacing, Span, TokenStream, TokenTree};
// use quote::{quote, ToTokens};
// use syn::{
//     self,
//     parse::Parse,
//     parse2, parse_macro_input,
//     visit_mut::{visit_expr_mut, VisitMut},
//     Block, Expr, ExprUnary, ItemFn, Local, ReturnType, Stmt, Token, Type, UnOp,
// };

// #[proc_macro]
// pub fn s(input : pm::TokenStream) -> pm::TokenStream {
//     let input = parse_macro_input!(input as Expr);

//     let output = sample_expr(input);

//     output.into()
// }

// fn substitute_custom_tilde_syntax(
//     input: pm::TokenStream,
// ) -> pm::TokenStream {
//     // let output = input.clone();
//     // println!("{:#?}", input.to_string());

//     fn replace_tildes(ts: TokenStream) -> TokenStream {
//         ts.into_iter()
//             .map(|t| match t {
//                 TokenTree::Group(g) => {
//                     Group::new(g.delimiter(), replace_tildes(g.stream()))
//                         .into_token_stream()
//                 }
//                 TokenTree::Punct(p) if p.as_char() == '%' => {
//                     quote! {
//                         *-*-*-*-
//                     }
//                 }
//                 _ => t.into_token_stream(),
//             })
//             .collect()
//     }

//     let intermediate = replace_tildes(input.into());

//     // println!("{:#?}", intermediate.to_string());

//     let mut function: ItemFn = parse2(intermediate).unwrap();

//     // println!("{:#?}", function);

//     IntermediateToSample.visit_item_fn_mut(&mut function);

//     let output = function.into_token_stream();

//     println!("{}", output.to_string());

//     output.into()
// }

// fn match_deref(e: &Expr) -> Option<&Expr> {
//     match e {
//         Expr::Unary(ExprUnary {
//             op: UnOp::Deref(_),
//             expr,
//             ..
//         }) => Some(expr),
//         _ => None,
//     }
// }

// fn match_neg(e: &Expr) -> Option<&Expr> {
//     match e {
//         Expr::Unary(ExprUnary {
//             op: UnOp::Neg(_),
//             expr,
//             ..
//         }) => Some(expr),
//         _ => None,
//     }
// }

// struct IntermediateToSample;

// impl VisitMut for IntermediateToSample {
//     fn visit_expr_mut(&mut self, e: &mut Expr) {
//         let b = Some(&*e)
//             .and_then(match_deref)
//             .and_then(match_neg)
//             .and_then(match_deref)
//             .and_then(match_neg)
//             .and_then(match_deref)
//             .and_then(match_neg)
//             .and_then(match_deref)
//             .and_then(match_neg);

//         // let b = Some(&*e)
//         //     .and_then(match_deref)
//         //     .and_then(match_neg);

//         // println!("{:?}", b);

//         match b {
//             Some(b) => {
//                 let mut b = b.clone();
//                 self.visit_expr_mut(&mut b);
//                 let b = sample_expr(b);
//                 *e = parse2(b).unwrap();
//             }
//             _ => {}
//         };
//     }
// }

// fn remove_tildes(ts: TokenStream) -> TokenStream {
//     ts.into_iter()
//         .map(|t| match t {
//             TokenTree::Group(g) => {
//                 Group::new(g.delimiter(), remove_tildes(g.stream()))
//                     .into_token_stream()
//             }
//             TokenTree::Punct(p) if p.as_char() == '-' => {
//                 quote! {
//                     ~
//                 }
//             }
//             _ => t.into_token_stream(),
//         })
//         .collect()
// }

// #[proc_macro_attribute]
// pub fn prob_test(
//     _attrs: pm::TokenStream,
//     input: pm::TokenStream,
// ) -> pm::TokenStream {
//     println!("{}", input.to_string());
//     let output : pm::TokenStream = remove_tildes(input.into()).into();
//     println!("{}", output.to_string());
//     output
// }
// #[proc_macro_attribute]
// pub fn prob_test(
//     _attrs: pm::TokenStream,
//     input: pm::TokenStream,
// ) -> pm::TokenStream {
//     let output = input.clone();
//     println!("{:#?}", input);

//     fn descend(ts: TokenStream) -> TokenStream {
//         let q = ts
//             .into_iter()
//             .map(|tt| {
//                 match tt {
//                     TokenTree::Group(g) => {
//                         // println!("{:?}", gts);
//                         Group::new(g.delimiter(), descend(g.stream()))
//                             .into_token_stream()
//                     }
//                     TokenTree::Punct(p)
//                         if p.as_char() == '~'
//                             && p.spacing() == Spacing::Alone =>
//                     {
//                         println!("Tilde encountered!");
//                         quote! {
//                             *-*-*-
//                         }
//                     }
//                     _ => tt.into_token_stream(),
//                 }
//             })
//             .collect();
//         q
//     }

//     fn descend3(ts: TokenStream) -> TokenStream {
//         let q = ts.into_iter().skip_while(|x| match x {
//             TokenTree::Punct(p)
//                 if p.as_char() == '~' && p.spacing() == Spacing::Alone =>
//             {
//                 true
//             }
//             TokenTree::Group(_) => true,
//             _ => false,
//         });

//         todo!()
//     }

//     let q: pm::TokenStream = descend(input.into()).into();
//     // println!("{:#?}", q);
//     let f = parse_macro_input!(q as ItemFn);
//     println!("{:?}", f);

//     fn descend2(ts: ItemFn) -> ItemFn {
//         let block = ts.block;
//         let stmts = block.stmts;
//         for s in stmts {
//             match s {
//                 Stmt::Local(mut l) => {
//                     l.init = if let Some((q, e)) = l.init {
//                         Some((q, Box::new(dodathing(*e))))
//                     } else {
//                         None
//                     };
//                     todo!()
//                 }
//                 Stmt::Item(i) => todo!(),
//                 Stmt::Expr(e) => todo!(),
//                 Stmt::Semi(e, s) => todo!(),
//             }
//         }

//         ItemFn {
//             attrs: ts.attrs,
//             vis: ts.vis,
//             sig: ts.sig,
//             block: todo!(),
//         }
//     }

//     fn dodathing(e: Expr) -> Expr {
//         todo!()
//     }

//     // let qq = quote! {
//     //         bernoulli(0.5)
//     // };

//     // let qqq = parse2::<Expr>(qq);
//     // println!("{:?}", qqq);

//     // let q: TokenStream = q.into_iter().map(descend).collect();

//     // println!("{:#?}", parse_macro_input!(input as ItemFn));
//     output
// }

// #[proc_macro]
// pub fn sample(input: pm::TokenStream) -> pm::TokenStream {
//     // let input = TokenStream::from(input);

//     let call = parse_macro_input!(input as ExprCall);
//     let func = call.func.to_token_stream().to_string();
//     let func_upper = {
//         let mut func = func.clone();
//         func.
//     };

//     let output = if PRIMITIVE_IDENTS
//         .contains(&func.as_str())
//     {
//         quote! {
//             let params = UniformParams(1., 2.);
//             let distribution =
//                 PrimitiveDistribution::Uniform(Uniform::new(params));
//             let name = tracing_path.global_name("y");
//             match trace_macro_injection(distribution, name, tracing_data) {
//                 PrimitiveSupportType::Uniform(result) => result,
//                 _ => unreachable!(),
//             }
//         }
//     } else {
//         todo!()
//     };
//     // pm::TokenStream::from(output)
//     output
// }

// fn prob_rec(token_tree: TokenTree) -> TokenTree {
//     match token_tree {
//         TokenTree::Group(g) => prob_rec2(g),
//         TokenTree::Ident(i) => prob_rec3(i),
//         t => t,
//     }
// }

// fn prob_rec2(group: Group) -> TokenTree {
//     let delimiter = group.delimiter();
//     let stream = group.stream();
//     TokenTree::Group(Group::new(delimiter, stream))
// }

// fn prob_rec3(ident: Ident) -> TokenTree {
//     let q = ident.to_string();
//     println!("{}", q);
//     TokenTree::Ident(ident)
// }
