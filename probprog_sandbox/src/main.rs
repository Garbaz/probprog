use probprog::{
    distributions::{
        bernoulli::{Bernoulli, BernoulliParams},
        uniform::{Uniform, UniformParams},
    },
    inference::{mcmc, MCMCConfig},
    macro_injection::trace_macro_injection,
    stats::{statistics::densities, visualization::simple_bar_graph},
    trace::{
        PrimitiveDistribution, PrimitiveSupportType, TracingData, TracingPath,
    },
};

/// A mock-up of how a probabilstic function would end up looking like
/// after being transformed by the macro.
/// Note: We should extract as much as possible from the function itself
/// into pre-written functions, such that the macro shenanigans are kept
/// at a minimum.
fn probfunc(
    mut tracing_path: TracingPath,
    tracing_data: &mut TracingData,
) -> f64 {
    {
        /* PROB MACRO CODE */
        tracing_path.descend("probfunc");
    }

    // USER CODE USER CODE USER CODE

    let x = {
        /* PROB MACRO CODE (Replaced `bernoulli(0.25)`) */
        let params = BernoulliParams(0.25);
        let distribution =
            PrimitiveDistribution::Bernoulli(Bernoulli::new(params));
        let name = tracing_path.global_name("x");
        match trace_macro_injection(distribution, name, tracing_data) {
            PrimitiveSupportType::Bernoulli(result) => result,
            _ => unreachable!(),
        }
    };

    if x {
        let y = {
            /* PROB MACRO CODE (Replaced `uniform(-2., -1.)`) */
            let params = UniformParams(-2., -1.);
            let distribution =
                PrimitiveDistribution::Uniform(Uniform::new(params));
            let name = tracing_path.global_name("y");
            match trace_macro_injection(distribution, name, tracing_data) {
                PrimitiveSupportType::Uniform(result) => result, // How do we do this part in the macro?
                _ => unreachable!(),
            }
        };
        y
    } else {
        let y = {
            /* PROB MACRO CODE (Replaced `uniform(-2., -1.)`) */
            let params = UniformParams(1., 2.);
            let distribution =
                PrimitiveDistribution::Uniform(Uniform::new(params));
            let name = tracing_path.global_name("y");
            match trace_macro_injection(distribution, name, tracing_data) {
                PrimitiveSupportType::Uniform(result) => result,
                _ => unreachable!(),
            }
        };
        y
    }
}

fn main() {
    let samples = 100000;
    let burn_in = samples / 4;
    let results = mcmc(MCMCConfig { samples, burn_in }, &probfunc);
    // println!("{:#?}",tracing_data);
    // println!("{:?}", results);
    let results = results.into_iter()/* .map(|x| OrderedFloat(x)) */;
    // let results = occurences(results);
    // println!("{:?}", results);
    // let results = normalize_map(results);
    // println!("{:?}", results);
    // println!("{:?}", results);
    let results = densities(-2.5..2.5, 80, results);
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
