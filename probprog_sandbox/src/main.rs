use probprog::{bernoulli, inference, o, prob, s, uniform, distribution::Distribution};

#[prob]
fn forward(p: f64, n: usize) -> Vec<bool> {
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(s!(bernoulli(p)))
    }
    v
}

#[prob]
fn backward(obs: Vec<bool>) -> f64 {
    let p = s!(uniform(0., 1.));

    for o in &obs {
        o!(bernoulli(p), o);
    }

    p
}


fn main() {


    let num_obs = 1000;
    let p = 0.1;
    let obs = forward(p, num_obs).sample().sample.value;

    let num_samples = 1000;
    let avg: f64 = inference(backward(obs))
        // Skip samples until we find a valid sample
        .skip_while(|s| s.log_probability.is_infinite()) 
        // Burn in the sampler to hopefully reach a stable point
        .skip(num_samples / 2) 
        // Take our samples 
        .take(num_samples)
        // And average them
        .map(|s| s.value / (num_samples as f64))
        .sum();
    println!("Coin weight:");
    println!("  real: {}, inferred: {}", p, avg);

    // const N: usize = 4;
    // let burn_in = N / 2;
    // let gen = metropolis_hastings(example3(vec![
    //     true, false, true, false, true, false,
    // ]));

    // let avg: f64 = gen
    //     .skip(burn_in)
    //     .take(N)
    //     .map(|s| (s.value as f64) / (N as f64))
    //     .sum();
    // println!("{}", avg);

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
//         let mut __log_probability = 0.;
//         let value = (|| {
//             let _q = {
//                 let s = uniform(-1., 1.).resample(__trace);
//                 __log_probability += s.log_probability;
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
//                             __log_probability += s.log_probability;
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
//                 __log_probability += s.log_probability;
//                 s.value
//             };

//             c
//         })();
//         Sample {
//             value,
//             log_probability: __log_probability,
//         }
//     }
// }
