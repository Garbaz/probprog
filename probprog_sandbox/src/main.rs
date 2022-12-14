use probprog::{
    bernoulli::{Bernoulli, BernoulliParams},
    distribution::Distribution,
    inference::InferenceConfig,
    trace::{Database, DatabaseEntry, TraceEntry},
};

/// A mock-up of how a probabilstic function would end up looking like
/// after being transformed by the macro.
/// Note: We should extract as much as possible from the function itself
/// into pre-written functions, such that the macro shenanigans are kept
/// at a minimum.
fn probfunc(trace: &mut InferenceConfig) -> u8 {
    {
        /* PROB MACRO CODE */
        trace.path += "probfunc/";
    }

    let x = {
        /* PROB MACRO CODE (Replaced `bernoulli(0.5)`) */
        let params = BernoulliParams { p: 0.5 };
        let name = trace.path.clone() + "x";
        let database_entry = match &trace.proposal {
            Some((n, entry)) if *n == name => Some(entry),
            _ => trace.tracedb.get(&name),
        };
        let value_t = match database_entry {
            Some(DatabaseEntry {
                trace_entry: TraceEntry::Bernoulli(ps, current),
                log_likelihood: current_ll,
            }) if *ps == params => {
                todo!()
            }
            Some(DatabaseEntry {
                trace_entry: TraceEntry::Bernoulli(ps, current),
                log_likelihood: current_ll,
            }) => {
                todo!("Handle case where we find the right type for the name, but it's the wrong params")
            }
            _ => {
                let distribution = Bernoulli::new(params).unwrap();
                let value = distribution.sample();
                let trace_entry = TraceEntry::Bernoulli(params, value);
                let database_entry = DatabaseEntry {
                    trace_entry,
                    log_likelihood: distribution.log_likelihood(value),
                };
                trace.tracedb.insert(name, database_entry);
                value
            }
        };
        value_t
    };
    if x {
        17
    } else {
        29
    }
}

fn main() {
    let mut trace = InferenceConfig::new(); // Initialize empty trace
    probfunc(&mut trace); // Initialize trace
    
    println!("{:#?}", trace);
    
    let wiggle_name = "probfunc/x".to_string(); // Pick "random" point in the trace to wiggle
    let &DatabaseEntry {
        trace_entry: TraceEntry::Bernoulli(params, current),
        log_likelihood: current_ll,
    } = trace.tracedb.get(&wiggle_name).unwrap(); // Look up that point in the initial trace
    let distribution = Bernoulli::new(params).unwrap();
    let proposal = distribution.kernel_propose(current); // Generate new proposal for that distribution
    let proposal_trace_entry = TraceEntry::Bernoulli(params, proposal);
    trace.proposal = Some((
        wiggle_name,
        DatabaseEntry {
            trace_entry: proposal_trace_entry,
            log_likelihood: 0.,
        },
    )); // Temporarily "insert" proposal into trace database

    println!("{:#?}", trace);

    probfunc(&mut trace); // Run again, this time with the proposal to calculate it's likelihood

    let (wiggle_name, proposal_database_entry) = trace.proposal.take().unwrap();

    let proposal_ll = proposal_database_entry.log_likelihood;
    let proposal_kernel_ll =
        distribution.kernel_log_likelihood(current, proposal);
    let current_kernel_ll =
        distribution.kernel_log_likelihood(proposal, current);
    let score =
        (proposal_ll + current_kernel_ll) - (current_ll + proposal_kernel_ll);

    if true || score > 0. || rand::random::<f64>().log2() < score {
        trace.tracedb.insert(wiggle_name, proposal_database_entry);
    }

    println!("{:#?}", trace);

    // let r = probfunc(&mut inference_config);
    // println!("{}\n{:#?}", r, inference_config);
}
