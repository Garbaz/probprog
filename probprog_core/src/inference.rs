use crate::{
    distribution::Distribution,
    __internal::trace::{TraceEntry, TracingData, TracingPathRec}, __internal::probfunc::ProbFunc,
};

use rand_distr as rd;

pub struct MCMCConfig {
    pub samples: usize,
    pub burn_in: usize,
}

/// Sample from the given probabilistic function with the Markov Chain Monte Carlo algorithm
pub fn mcmc<F, B>(config: MCMCConfig, prob_func: &mut ProbFunc<B, F>) -> Vec<B>
where
    F: Fn(&mut TracingPathRec, &mut TracingData) -> B,
{
    let mut results: Vec<B> = Vec::new();
    let mut tracing_data = TracingData::new(); // Create empty trace
    (prob_func.0)(&mut TracingPathRec::new(), &mut tracing_data); // Initialize the trace
    for i in 0..config.samples {
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
            },
        ));
        // Temporarily "insert" proposal into trace database

        let previous_trace_log_likelihood = tracing_data.trace_log_likelihood;
        tracing_data.trace_log_likelihood = 0.;

        // Run again, this time with the proposal to calculate it's likelihood
        {
            let r = (prob_func.0)(&mut TracingPathRec::new(), &mut tracing_data);
            if i > config.burn_in {
                results.push(r);
            }
        }

        let (wiggle_name, proposal_database_entry) =
            tracing_data.proposal.take().unwrap();

        let forward_kernel_ll =
            distribution.kernel_log_likelihood(current, proposal);
        let backward_kernel_ll =
            distribution.kernel_log_likelihood(proposal, current);

        // The Metropoli-Hastings accept ratio
        let score = tracing_data.trace_log_likelihood
            - previous_trace_log_likelihood
            + backward_kernel_ll
            - forward_kernel_ll;

        // Stochastically decide whether to accept or reject the proposed change
        // to our trace
        if score > 0. || rand::random::<f64>().log2() < score {
            tracing_data
                .trace
                .insert(wiggle_name, proposal_database_entry);
        } else {
            tracing_data.trace_log_likelihood = previous_trace_log_likelihood;
        }
    }

    results
}
