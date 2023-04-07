use experiment::new_structure2::{bernoulli, FnProb, Sample, uniform};
use experiment::{
    new_structure2::Distribution,
    trace::{Trace, TraceDirectory, TracedSample},
};

pub fn t4(p: f64) -> impl FnProb<usize> {
    move || {
        let mut __log_likelihood = 0.;
        let __trace = &mut Trace::new();
        let __value = (|| {

            let q = {
                let ts = uniform(-1.,1.).sample_traced();

                __log_likelihood += ts.sample.log_likelihood;
                *__trace += ts.trace;

                ts.sample.value
            };

            let mut c = 0;
            {
                let mut __loop_counter: usize = 0;
                loop {
                    let __trace =
                        __trace.descend(TraceDirectory::Loop(__loop_counter));

                    {
                        let x = {
                            let ts = bernoulli(p).sample_traced();

                            __log_likelihood += ts.sample.log_likelihood;
                            *__trace += ts.trace;

                            ts.sample.value
                        };

                        if x {
                            return c;
                        } else {
                            c += 1;
                        }
                    }
                    __loop_counter += 1;
                }
            }
        })();
        TracedSample {
            sample: Sample {
                value: __value,
                log_likelihood: __log_likelihood,
            },
            trace: __trace.to_owned(),
        }
    }
}

fn main() {
    let pf = t4(0.33);
    // for _ in 0..3 {
    println!("{:#?}", pf.sample_traced());
    // }
}

// fn t3(p: f64) -> impl FnProb<usize> {
//     move || -> TracedSample<usize> {
//         let mut __trace_data = TraceData::new();
//         let __value = (|| {
//             let mut c = 0;
//             {
//                 || {
//                 let mut __loop_counter = 0;
//                 loop {
//                     let (__value, __subtrace_data) = {
//                         let mut __trace_data = TraceData::new();
//                         let __value = (|| {
//                             let x = {
//                                 let ts = bernoulli(p).sample_traced();
//                                 __trace_data += ts.trace_data;
//                                 ts.value
//                             };

//                             if x {
//                                 return c;
//                             } else {
//                                 c += 1;
//                             }
//                         })();
//                         (__value, __trace_data)
//                     };

//                     __loop_counter += 1;
//                 }
//             }
//         })();

//         TracedSample {
//             value: __value,
//             trace_data: __trace_data,
//         }
//     }
// }}

// fn sum_uniform(n: usize) -> impl FnProb<f64> {
//     // let mut __traced_sample = TracedSample::new((), 0.);
//     move || -> TracedSample<f64> {
//         let mut __log_likelihood = 0.;
//         let mut __trace = Trace::new();
//         let __trace_directory =
//             TraceDirectory::Function("sum_uniform".to_string());
//         let __value = || -> f64 {
//             let mut sum = 0.;
//             {
//                 let __trace_directory = TraceDirectory::Loop(0);
//                 let mut __trace = Trace::new();
//                 let __value = (|| {
//                     for _ in 0..n {
//                         let x = {
//                             let traced_sample =
//                                 uniform(-1., 1.).sample_traced();
//                             __trace.attach(directory, subtrace);
//                             traced_sample.sample.value
//                         };
//                         sum += x;
//                     }
//                 })();
//                 __value
//             }

//             let x = {
//                 uniform(-1., 1.).sample_traced();
//             };

//             todo!()
//         }();
//         TracedSample {
//             sample: Sample {
//                 value: __value,
//                 log_likelihood: __log_likelihood,
//             },
//             trace: __trace,
//         }
//     }
// }

/* fn t2(p: f64) -> impl FnProb<usize> {
       move || -> TracedSample<usize> {
           let mut __log_likelihood = 0.;
           let __trace = &mut Trace::new();

           let __value = (|| -> usize {
               let mut c = 0;
               {
                   let __loop_counter = 0;
                   loop {
                       let __trace = {
                           let subtrace = &mut Trace::new();
                       };

                       let x = {
                           let traced_sample = bernoulli(p).sample_traced();

                           let q = || 7;

                           traced_sample.sample.value
                       };
                       if x {
                           return c;
                       } else {
                           c += 1;
                       }
                   }
               }
           })();

           TracedSample {
               sample: Sample {
                   value: __value,
                   log_likelihood: __log_likelihood,
               },
               trace: __trace.to_owned(),
           }
       }
   }
*/
