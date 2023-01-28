use probprog::{
    inference::{mcmc, MCMCConfig},
    primitives::bernoulli,
    prob, s,
    stats::{statistics::densities, visualization::simple_bar_graph},
};

#[prob]
fn testfunc() -> f64 {
    // let mut __probprog_tracing_path =
    //     __probprog_tracing_path.descend_function("testfunc");

    let mut s = 0.;

    // {
    // let mut __probprog_tracing_path =
    //     __probprog_tracing_path.descend_loop();
    for _ in 0..3 {
        // {
        //     let mut __probprog_tracing_path =
        //         __probprog_tracing_path.descend_loop();
        for _ in 0..3 {
            if s!(bernoulli(0.5)) {
                s += 1.;
            } else {
                s -= 1.;
            }

            //     __probprog_tracing_path.increment_loop();
        }
        // }
        // __probprog_tracing_path.increment_loop();
    }
    // }

    // let __probprog_postponed_result = {
    //     if x {
    //     let y = sample!(uniform(-2., -1.));
    //     y
    //     } else {
    //         let y = sample!(uniform(1., 2.));
    //         y
    //     }
    // };
    s
}

// #[prob]
// fn testfunc2(p: f64) -> bool {
//     let mut __probprog_tracing_path =
//         __probprog_tracing_path.descend_function("testfunc2");
//     sample!(bernoulli(p))
// }

// #[prob]
// fn testfunc4() -> f64 {
//     let mut __probprog_tracing_path =
//         __probprog_tracing_path.descend_function("testfunc4");
//     if sample!(bernoulli(0.1)) {
//         sample!(testfunc4()) + 1.
//     } else {
//         0.
//     }
// }

// fn testfunc3(__tracing_path: TracingPathRec) {
//     let mut __tracing_path = __tracing_path.descend_function("testfunc3");

//     /* sample something here with path '/testfunc3/' */
//     println!("{:?}", __tracing_path.variable());
//     println!("{:?}", __tracing_path.variable());

//     {
//         let mut __tracing_path = __tracing_path.descend_loop();
//         for _ in 0..5 {
//             println!("{:?}", __tracing_path.variable());
//             println!("{:?}", __tracing_path.variable());
//             __tracing_path.increment_loop();
//         }
//     }

//     /* sample some more here, again with only path '/testfunc3/'*/
//     println!("{:?}", __tracing_path.variable());
// }

// #[prob]
// fn probfunc2(x: f64, y: f64) -> f64 {
//     let r = bernoulli(0.33).sample();
//     if r {
//         x
//     } else {
//         y
//     }
// }

// #[prob]
// fn testfunc5() {
//     let mut __probprog_tracing_path =
//         __probprog_tracing_path.descend_function("testfunc5");
//     let q = sm!(bernoulli(0.5));
// }

fn main() {
    // testfunc3(TracingPathRec::new());

    let samples = 10000;
    let burn_in = samples / 4;
    let results = mcmc(MCMCConfig { samples, burn_in }, &mut testfunc());
    // println!("{:#?}",tracing_data);
    // println!("{:?}", results);
    let results = results.into_iter()/* .map(|x| OrderedFloat(x)) */;
    // let results = occurences(results);
    // println!("{:?}", results);
    // let results = normalize_map(results);
    // println!("{:?}", results);
    // println!("{:?}", results);
    let results = densities(0.0..10.0, 50, results);
    // println!("{:?}", results);
    println!("{}", simple_bar_graph(16, &results));

    // let mut tracing_data = TracingData::new(); // Initialize empty trace
    // probfunc(String::new(), &mut tracing_data); // Initialize trace

    // println!("{:#?}", tracing_data);

    // let wiggle_name = "probfunc/x".to_string(); // Pick "random" point in the trace to wiggle
    // let &TraceEntry::Bernoulli(TraceEntryValues {
    //     params,
    //     value: current,
    //     log_likelihood: current_ll,
    // }) = tracing_data.trace.get(&wiggle_name).unwrap(); // Look up that point in the initial trace
    // let distribution = Bernoulli::new(params).unwrap();
    // let proposal = distribution.kernel_propose(current); // Generate new proposal for that distribution
    // let proposal_trace_values = TraceEntryValues {
    //     params,
    //     value: proposal,
    //     log_likelihood: 0.,
    // };
    // tracing_data.proposal =
    //     Some((wiggle_name, TraceEntry::Bernoulli(proposal_trace_values))); // Temporarily "insert" proposal into trace database

    // // println!("{:#?}", tracing_data);

    // probfunc(String::new(), &mut tracing_data); // Run again, this time with the proposal to calculate it's likelihood

    // let (wiggle_name, proposal_database_entry) =
    //     tracing_data.proposal.take().unwrap();

    // let TraceEntry::Bernoulli(TraceEntryValues {
    //     log_likelihood: proposal_ll,
    //     ..
    // }) = proposal_database_entry;
    // let proposal_kernel_ll =
    //     distribution.kernel_log_likelihood(current, proposal);
    // let current_kernel_ll =
    //     distribution.kernel_log_likelihood(proposal, current);
    // let score =
    //     (proposal_ll + current_kernel_ll) - (current_ll + proposal_kernel_ll);

    // if score > 0. || rand::random::<f64>().log2() < score {
    //     println!("Accepting proposal!");
    //     tracing_data
    //         .trace
    //         .insert(wiggle_name, proposal_database_entry);
    // }

    // println!("{:#?}", tracing_data);

    // let r = probfunc(&mut inference_config);
    // println!("{}\n{:#?}", r, inference_config);
}
