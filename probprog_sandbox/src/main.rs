use probprog::{bernoulli, inference, o, prob, s, uniform};

#[prob]
fn probfunc1(p: f64) -> usize {
    let mut c = 0;
    loop {
        let x = s!(bernoulli(p));

        if x {
            c += 1;
        } else {
            return c;
        }
    }
}

#[prob]
fn probfunc2(obs: Vec<bool>) -> f64 {
    let p = s!(uniform(0., 1.));

    for o in &obs {
        o!(bernoulli(p), o);
    }

    p
}

fn main() {
    let n = 1000;
    let burn_in = n / 2;

    let f = probfunc2(vec![true, false, true, true]);

    let samples: Vec<_> = inference(f).skip(burn_in).take(n).collect();
    let avg: f64 = samples.iter().map(|x| (*x as f64) / (n as f64)).sum();
    println!("{}", avg);

    // println!(
    //     "{:#?}",
    //     f(Trace::Function {
    //         name: "probfunc1".to_string(),
    //         subtraces: Traces::from(vec![
    //             Trace::Loop {
    //                 iteration: 0,
    //                 subtraces: Traces::from(vec![Trace::Sample {
    //                     sample: Sample {
    //                         value: ParametrizedValue::Bernoulli {
    //                             value: true,
    //                             p: 0.25
    //                         },
    //                         log_probability: -2.0,
    //                     }
    //                 }])
    //             },
    //             Trace::Loop {
    //                 iteration: 1,
    //                 subtraces: Traces::from(vec![Trace::Sample {
    //                     sample: Sample {
    //                         value: ParametrizedValue::Bernoulli {
    //                             value: true,
    //                             p: 0.25
    //                         },
    //                         log_probability: -2.0,
    //                     }
    //                 }])
    //             }
    //         ])
    //     })
    // );
}

// fn probfunc1(p: f64) -> impl FnProb<usize> {
//     const __FUNCTION_NAME: &str = "probfunc1";
//     move |__old_trace: Trace| -> TracedSample<usize> {
//         let mut __new_trace = Trace::Function {
//             name: __FUNCTION_NAME.to_string(),
//             subtraces: Traces::new(),
//         };

//         let mut __total_log_probability = 0.;

//         let return_value = (|| -> usize {
// let mut __old_traces =
//     __old_trace.function_subtraces(__FUNCTION_NAME);
// let __new_traces = __new_trace.subtraces().unwrap();

//             let mut c = 0;

//             {
//                 let mut __loop_counter = 0;
//                 loop {
//                     let (mut __old_traces, __new_traces) =
//                         __inject::loop_descend(
//                             &mut __old_traces,
//                             __new_traces,
//                             __loop_counter,
//                         );

//                     let x = __inject::sample(
//                         &mut __old_traces,
//                         __new_traces,
//                         &mut __total_log_probability,
//                         bernoulli(p),
//                     );

//                     if x {
//                         c += 1;
//                     } else {
//                         __inject::sample(
//                             &mut __old_traces,
//                             __new_traces,
//                             &mut __total_log_probability,
//                             probfunc2(),
//                         );
//                         __inject::sample(
//                             &mut __old_traces,
//                             __new_traces,
//                             &mut __total_log_probability,
//                             probfunc2(),
//                         );
//                         __inject::sample(
//                             &mut __old_traces,
//                             __new_traces,
//                             &mut __total_log_probability,
//                             probfunc2(),
//                         );
//                         return c;
//                     }

//                     __loop_counter += 1;
//                 }
//             }
//         })();

//         TracedSample {
//             sample: Sample {
//                 value: return_value,
//                 log_probability: __total_log_probability,
//             },
//             trace: __new_trace,
//         }
//     }
// }

// fn probfunc2() -> impl FnProb<()> {
//     const __FUNCTION_NAME: &str = "probfunc2";
//     move |__old_trace: Trace| -> TracedSample<()> {
//         let mut __new_trace = Trace::Function {
//             name: __FUNCTION_NAME.to_string(),
//             subtraces: Traces::new(),
//         };

//         let mut __total_log_probability = 0.;

//         let return_value = (|| {})();

//         TracedSample {
//             sample: Sample {
//                 value: return_value,
//                 log_probability: __total_log_probability,
//             },
//             trace: __new_trace,
//         }
//     }
// }
