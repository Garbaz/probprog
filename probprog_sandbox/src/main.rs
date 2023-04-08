use probprog::inference::metropolis_hastings;
use probprog::primitive::{bernoulli, uniform};
use probprog::{o, s, prob};

#[prob]
fn example3(obs: Vec<bool>) -> f64 {
    let p = s!(uniform(0., 1.));

    for o in &obs {
        o!(bernoulli(p), o);
    }

    p
}

fn main() {
    const N: usize = 10000;
    let burn_in = N / 2;
    let gen =
        metropolis_hastings(example3(vec![true, false, true, false, true, false]));

    let avg: f64 = gen
        .skip(burn_in)
        .take(N)
        .map(|s| (s.value as f64) / (N as f64))
        .sum();
    println!("{}", avg);

    // let gen = mh(t4(0.33));
    // const N: usize = 1000;
    // let avg: f64 = gen
    //     .skip(100)
    //     .take(N)
    //     .map(|s| (s.value as f64) / (N as f64))
    //     .sum();
    // println!("{}", avg);
}

// pub fn t4(p: f64) -> impl FnProb<usize> {
//     move |__trace: &mut Trace| -> Sample<usize> {
//         let mut __log_likelihood = 0.;
//         let value = (|| {
//             let _q = {
//                 let s = uniform(-1., 1.).resample(__trace);
//                 __log_likelihood += s.log_likelihood;
//                 s.value
//             };

//             let mut c = 0;

//             {
//                 let mut __loop_counter: usize = 0;
//                 loop {
//                     let __trace =
//                         __trace.descend(TraceDirectory::Loop(__loop_counter));

//                     {
//                         let x = {
//                             let s = bernoulli(p).resample(__trace);
//                             __log_likelihood += s.log_likelihood;
//                             s.value
//                         };

//                         if x {
//                             // return c;
//                             break;
//                         } else {
//                             c += 1;
//                         }
//                     }

//                     __loop_counter += 1;
//                 }
//             }

//             let _p = {
//                 let s = uniform(-2., 2.).resample(__trace);
//                 __log_likelihood += s.log_likelihood;
//                 s.value
//             };

//             c
//         })();
//         Sample {
//             value,
//             log_likelihood: __log_likelihood,
//         }
//     }
// }
