use probprog::{
    distributions::bernoulli::{Bernoulli, BernoulliParams},
    distribution::Distribution,
    inference::{mcmc, MCMCConfig},
    statistics::{normalize, occurences},
    trace::{DistributionAndValue, TraceEntry, TracingData, TracingPath},
};

/// A mock-up of how a probabilstic function would end up looking like
/// after being transformed by the macro.
/// Note: We should extract as much as possible from the function itself
/// into pre-written functions, such that the macro shenanigans are kept
/// at a minimum.
fn probfunc(
    mut tracing_path: TracingPath,
    tracing_data: &mut TracingData,
) -> i32 {
    {
        /* PROB MACRO CODE */
        tracing_path.descend("probfunc");
    }

    // USER CODE USER CODE USER CODE

    let x = {
        /* PROB MACRO CODE (Replaced `bernoulli(0.25)`) */
        let params = BernoulliParams(0.25);
        let distribution = Bernoulli::new(params);
        let name = tracing_path.global_name("x_1");
        externalized_macro_injection(&distribution, name, tracing_data)
    };

    if x {
        17
    } else {
        29
    }
}

fn externalized_macro_injection<D: Distribution + 'static>(
    distribution: &D,
    // &impl Distribution<
    //     ParamsType = ParamsType,
    //     SupportType = SupportType,
    // >,
    name: TracingPath,
    tracing_data: &mut TracingData,
) -> D::SupportType {
    let database_entry = match &tracing_data.proposal {
        // If there is a proposal, and it is for us, take it
        Some((n, entry)) if *n == name => Some(entry),
        // Otherwise, try looking in the trace for our entry
        _ => tracing_data.trace.get(&name),
    };
    match database_entry {
        Some(trace_entry)
            if trace_entry
                .distribution_and_value
                .distribution_and_value()
                .params_eq(distribution) =>
        {
            // ^ The random choice in the database with our name has sampled
            // the same distribution with the same parameters.

            tracing_data.trace_log_likelihood += trace_entry.log_likelihood;

            // We can downcast and unwrap here because `params_eq` in the match guard ensures that we get `Some`.
            // This should probably be handled cleaner!
            let d = trace_entry
                .distribution_and_value
                .distribution_and_value()
                .as_any()
                .downcast_ref::<DistributionAndValue<D>>()
                .unwrap();
            d.value
        }
        Some(trace_entry)
            if trace_entry
                .distribution_and_value
                .distribution_and_value()
                .kind_eq(distribution) =>
        {
            // ^ The random choice in the database with our name has sampled
            // the same distribution, but with different parameters.
            // We reuse the value, but have to calculate a new log likelihood.

            let d = trace_entry
                .distribution_and_value
                .distribution_and_value()
                .as_any()
                .downcast_ref::<DistributionAndValue<D>>()
                .unwrap();
            let value = d.value;
            let log_likelihood = distribution.log_likelihood(value);
            tracing_data.trace.insert(
                name,
                TraceEntry {
                    distribution_and_value: distribution.trace(value),
                    log_likelihood,
                },
            );
            tracing_data.trace_log_likelihood += log_likelihood;
            value
        }
        _ => {
            // ^ There either was no random choice in the database with our name,
            // or it was of the wrong type. So we sample a fresh value and insert
            // it into the database.

            // let distribution = Bernoulli::new(params).unwrap();
            let value = distribution.sample();
            let log_likelihood = distribution.log_likelihood(value);
            let trace_entry = TraceEntry {
                distribution_and_value: distribution.trace(value),
                log_likelihood,
            };
            tracing_data.trace.insert(name, trace_entry);
            tracing_data.trace_log_likelihood += log_likelihood;
            value
        }
    }
}

fn main() {
    let results = mcmc(
        MCMCConfig {
            samples: 10000,
            burn_in: 1000,
        },
        &probfunc,
    );
    // println!("{:#?}",tracing_data);
    // println!("{:?}", results);
    let results = occurences(results);
    println!("{:?}", results);
    let results = normalize(results);
    println!("{:?}", results);

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
