extern crate proc_macro;

use proc_macro as pm;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    self, parse_macro_input, Block, ItemFn, ReturnType, Token, Type,
};

#[proc_macro_attribute]
pub fn prob(_attrs: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    // let input = TokenStream::from(input);
    let code = input.clone();
    // println!("{:#?}", code);

    let mut func = parse_macro_input!(code as ItemFn);

    // println!("{:#?}", func);

    let mut rarrow = Token![->](Span::call_site());
    let orig_func_output = match func.sig.output {
        ReturnType::Default => quote! {()},
        ReturnType::Type(a, t) => {
            rarrow = a;
            t.into_token_stream()
        }
    };

    // println!("{:#?}", orig_func_output);
    // println!("{}", orig_func_output.to_token_stream().to_string());

    let new_func_output = quote! {
        ::probprog::__internal::probfunc::ProbFunc<(#orig_func_output),
                 impl Fn(&mut ::probprog::__internal::trace::TracingPathRec,
                         &mut ::probprog::__internal::trace::TracingData) -> (#orig_func_output)>
    }.into();

    // println!("{}", new_func_output.to_string());

    let new_func_output = parse_macro_input!(new_func_output as Type);

    func.sig.output = ReturnType::Type(rarrow, Box::new(new_func_output));

    // println!("{}", func.to_token_stream().to_string());

    let old_func_block = func.block.into_token_stream();
    let new_func_block = quote! {
        {
            ::probprog::__internal::probfunc::ProbFunc::new(
                move | __probprog_tracing_path: &mut ::probprog::__internal::trace::TracingPathRec,
                       __probprog_tracing_data: &mut ::probprog::__internal::trace::TracingData |
                    #old_func_block
            )
        }
    }
    .into();
    let new_func_block = parse_macro_input!(new_func_block as Block);
    func.block = Box::new(new_func_block);

    // println!("{}", func.to_token_stream().to_string());

    // let output: TokenStream = code.into_iter().map(prob_rec).collect();

    // let attrs = Punctuated::<TokenStream, Token![,]>::parse_terminated(attrs);
    // println!("{:#?}", attrs);
    // let ast = input.clone();
    // println!("{:#?}", ast);
    // let ast = parse_macro_input!(ast as ItemFn);
    // let stmts = ast.block.stmts;
    // println!("{:#?}", stmts);
    // for s in stmts {
    //     match s {
    //         Stmt::Local(x) => {
    //             println!("{:?}", x.init.unwrap().1);
    //         }
    //         Stmt::Item(x) => {
    //             // ?
    //         }
    //         Stmt::Expr(x) => {
    //         }
    //         Stmt::Semi(x, y) => {
    //             /* Do same with `x` as in `Stmt::Expr` */
    //         }
    //     }
    // }
    // ast.to_token_stream().into()
    // pm::TokenStream::from(input)
    func.into_token_stream().into()
}

#[proc_macro]
pub fn sample(input: pm::TokenStream) -> pm::TokenStream {
    let input = TokenStream::from(input);
    //    let code = input.clone();

    let output = quote! {
        ::probprog::__internal::probfunc::traced_sample(
            &mut (#input),
            &mut __probprog_tracing_path,
            __probprog_tracing_data,
        )
    };

    //    println!("{}", code.to_string());

    pm::TokenStream::from(output)
    //    output
}

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
