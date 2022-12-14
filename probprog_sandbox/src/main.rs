use probprog::{
    bernoulli::{Bernoulli, BernoulliParams},
    distribution::Distribution,
    inference::TracingData,
    trace::{Trace, TraceEntry, TraceValues},
};

/// A mock-up of how a probabilstic function would end up looking like
/// after being transformed by the macro.
/// Note: We should extract as much as possible from the function itself
/// into pre-written functions, such that the macro shenanigans are kept
/// at a minimum.
fn probfunc(mut tracing_path: String, tracing_data: &mut TracingData) -> u8 {
    {
        /* PROB MACRO CODE */
        tracing_path += "probfunc/";
    }

    // USER CODE USER CODE USER CODE

    let x = {
        /* PROB MACRO CODE (Replaced `bernoulli(0.5)`) */
        let params = BernoulliParams { p: 0.5 };
        let name = tracing_path.clone() + "x";
        let database_entry = match &tracing_data.proposal {
            Some((n, entry)) if *n == name => Some(entry),
            _ => tracing_data.trace.get(&name),
        };
        let value_t = match database_entry {
            Some(TraceEntry::Bernoulli(TraceValues {
                params: trace_params,
                value,
                log_likelihood,
            })) if *trace_params == params => {
                todo!("Handle case where we find the right type and params match")
            }
            Some(TraceEntry::Bernoulli(TraceValues {
                params: trace_params,
                value,
                log_likelihood,
            })) => {
                todo!("Handle case where we find the right type but params don't match")
            }
            _ => {
                let distribution = Bernoulli::new(params).unwrap();
                let value = distribution.sample();
                let trace_entry = TraceEntry::Bernoulli(TraceValues {
                    params,
                    value,
                    log_likelihood: distribution.log_likelihood(value),
                });
                tracing_data.trace.insert(name, trace_entry);
                value
            }
        };
        value_t
        /* END PROB MACRO CODE (Replaced `bernoulli(0.5)`) */
    };

    if x {
        17
    } else {
        29
    }
}

fn main() {
    let mut tracing_data = TracingData::new(); // Initialize empty trace
    probfunc(String::new(), &mut tracing_data); // Initialize trace

    println!("{:#?}", tracing_data);

    let wiggle_name = "probfunc/x".to_string(); // Pick "random" point in the trace to wiggle
    let &TraceEntry::Bernoulli(TraceValues {
        params,
        value: current,
        log_likelihood: current_ll,
    }) = tracing_data.trace.get(&wiggle_name).unwrap(); // Look up that point in the initial trace
    let distribution = Bernoulli::new(params).unwrap();
    let proposal = distribution.kernel_propose(current); // Generate new proposal for that distribution
    let proposal_trace_values = TraceValues {
        params,
        value: proposal,
        log_likelihood: 0.,
    };
    tracing_data.proposal =
        Some((wiggle_name, TraceEntry::Bernoulli(proposal_trace_values))); // Temporarily "insert" proposal into trace database

    println!("{:#?}", tracing_data);

    probfunc(String::new(), &mut tracing_data); // Run again, this time with the proposal to calculate it's likelihood

    let (wiggle_name, proposal_database_entry) =
        tracing_data.proposal.take().unwrap();

    let TraceEntry::Bernoulli(TraceValues {
        log_likelihood: proposal_ll,
        ..
    }) = proposal_database_entry;
    let proposal_kernel_ll =
        distribution.kernel_log_likelihood(current, proposal);
    let current_kernel_ll =
        distribution.kernel_log_likelihood(proposal, current);
    let score =
        (proposal_ll + current_kernel_ll) - (current_ll + proposal_kernel_ll);

    if true || score > 0. || rand::random::<f64>().log2() < score {
        tracing_data
            .trace
            .insert(wiggle_name, proposal_database_entry);
    }

    println!("{:#?}", tracing_data);

    // let r = probfunc(&mut inference_config);
    // println!("{}\n{:#?}", r, inference_config);
}
