use crate::{
    __internal::probfunc::ProbFunc,
    __internal::{
        probfunc::ProbFn,
        trace::{TraceEntry, TracingData, TracingPathRec},
    },
    distribution::Distribution,
};

use rand_distr as rd;

#[derive(Debug, Clone)]
pub struct MCMCConfig {
    pub samples: usize,
    pub burn_in: usize,
    pub init_attempts: usize,
}

#[derive(Debug, Clone)]
pub enum MCMCError {
    InitFailed,
}

#[derive(Debug, Clone)]
pub struct MCMCReport {
    pub init_failures: usize,
    pub sample_failures: usize,
}

/// Sample from the given probabilistic function with the Markov Chain Monte Carlo algorithm
pub fn mcmc<T, F: ProbFn<T>>(
    config: MCMCConfig,
    prob_func: &mut ProbFunc<T, F>,
) -> Result<(Vec<T>, MCMCReport), MCMCError> {
    let mut results: Vec<T> = Vec::new();
    let mut tracing_data = TracingData::new(); // Create empty trace

    let mut init_failures = 0;

    // Initialize the trace
    {
        while let Err(_err) =
            (prob_func.0)(&mut TracingPathRec::new(), &mut tracing_data)
        {
            init_failures += 1;
            if init_failures > config.init_attempts {
                return Err(MCMCError::InitFailed);
            }
            tracing_data = TracingData::new();
        }
    }

    let mut sample_total = 0;
    let mut sample_successes = 0;

    // Take samples
    while sample_successes < config.samples + config.burn_in {
        sample_total += 1;

        // Pick a random point in the trace to wiggle
        let wiggle_path = {
            let names: Vec<_> = tracing_data.trace.keys().collect();
            let name_sampler = rd::Uniform::new(0, names.len());
            let wiggle_index = rd::Distribution::sample(
                &name_sampler,
                &mut rand::thread_rng(),
            );
            names[wiggle_index]
        };

        // Look up that point in the initial trace
        let TraceEntry {
            distribution,
            value: current,
            ..
        } = tracing_data.trace.get(wiggle_path).unwrap().clone();
        // ^ We can unwrap here because we know `wiggle_name` is a valid key

        // Generate new proposal for that distribution
        let proposal = distribution.kernel_propose(current);

        tracing_data.proposal = Some((
            wiggle_path.clone(),
            TraceEntry {
                distribution: distribution.clone(),
                value: proposal,
                log_likelihood: 0.,
                touched: false,
            },
        ));
        // Temporarily "insert" proposal into trace database

        // let previous_trace_log_likelihood =
        //     tracing_data.trace_log_likelihood;
        let previous_tracing_data = tracing_data.clone();
        tracing_data.trace_log_likelihood = 0.;

        // Run again, this time with the proposal to calculate it's likelihood
        if let Ok(result) =
            (prob_func.0)(&mut TracingPathRec::new(), &mut tracing_data)
        {
            // Throw away burn-in sample, otherwise store sample
            sample_successes += 1;
            if sample_successes > config.burn_in {
                results.push(result);
            }

            let (wiggle_name, proposal_database_entry) =
                tracing_data.proposal.take().unwrap();
            //^ This this `unwrap` safe?

            let forward_kernel_ll =
                distribution.kernel_log_likelihood(current, proposal);
            let backward_kernel_ll =
                distribution.kernel_log_likelihood(proposal, current);

            // The Metropoli-Hastings accept ratio
            let score = tracing_data.trace_log_likelihood
                - previous_tracing_data.trace_log_likelihood
                + backward_kernel_ll
                - forward_kernel_ll;

            // Stochastically decide whether to accept or reject the proposed change
            // to our trace
            if score > 0. || rand::random::<f64>().log2() < score {
                tracing_data
                    .trace
                    .insert(wiggle_name, proposal_database_entry);

                tracing_data.clean_trace(); // ?
            } else {
                tracing_data = previous_tracing_data;
                //^ ?
            }
        } else {
            tracing_data = previous_tracing_data;
            //^ Is this the right thing to do?
        }
    }

    Ok((
        results,
        MCMCReport {
            init_failures,
            sample_failures: sample_total - sample_successes,
        },
    ))
}
