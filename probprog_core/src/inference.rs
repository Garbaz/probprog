use std::collections::BTreeMap;

use crate::{
    bernoulli::Bernoulli,
    distribution::Distribution,
    trace::{TraceEntry, TraceEntryValues},
};

use rand_distr as rd;

#[derive(Debug)]
pub struct TracingData {
    // pub path: String,
    pub trace: BTreeMap<String, TraceEntry>,
    pub proposal: Option<(String, TraceEntry)>,
    pub total_log_likelihood: f64,
    // pub proposal_log_likelihood: f64,
}

impl TracingData {
    // pub fn new(proposal_name : String, proposal: DatabaseEntry) -> Self {
    //     InferenceConfig {
    //         tracedb: Database::new(),
    //         path: String::new(),
    //         proposal: Some((proposal_name, proposal)),
    //     }
    // }

    pub fn new() -> Self {
        Self {
            // path: String::new(),
            trace: BTreeMap::new(),
            proposal: None,
            total_log_likelihood: 0.,
        }
    }
}

pub struct MCMCConfig {
    pub samples: usize,
    pub burn_in: usize,
}

pub fn mcmc<F, B>(config: MCMCConfig, prob_thunk: F) -> Vec<B>
where
    F: Fn(String, &mut TracingData) -> B,
    // B: std::cmp::Ord,
{
    let mut results: Vec<B> = Vec::new();
    let mut tracing_data = TracingData::new(); // Create empty trace
    prob_thunk(String::new(), &mut tracing_data); // Initialize the trace
    for i in 0..config.samples {
        // Pick a random point in the trace to wiggle
        let wiggle_name = {
            let names: Vec<&String> = tracing_data.trace.keys().collect();
            let name_sampler = rd::Uniform::new(0, names.len());
            let wiggle_index = rd::Distribution::sample(
                &name_sampler,
                &mut rand::thread_rng(),
            );
            names[wiggle_index]
        };

        // Look up that point in the initial trace
        let &TraceEntry::Bernoulli(TraceEntryValues {
            params,
            value: current,
            // log_likelihood: current_ll,
            ..
        }) = tracing_data.trace.get(wiggle_name).unwrap();

        let distribution = Bernoulli::new(params).unwrap();

        // Generate new proposal for that distribution
        let proposal = distribution.kernel_propose(current);
        let proposal_trace_values = TraceEntryValues {
            params,
            value: proposal,
            log_likelihood: 0.,
        };
        tracing_data.proposal = Some((
            wiggle_name.clone(),
            TraceEntry::Bernoulli(proposal_trace_values),
        )); // Temporarily "insert" proposal into trace database

        let previous_total_log_likelihood = tracing_data.total_log_likelihood;
        tracing_data.total_log_likelihood = 0.;

        // Run again, this time with the proposal to calculate it's likelihood
        {
            let r = prob_thunk(String::new(), &mut tracing_data);
            if i > config.burn_in {
                results.push(r);
            }
        }

        let (wiggle_name, proposal_database_entry) =
            tracing_data.proposal.take().unwrap();

        // let TraceEntry::Bernoulli(TraceEntryValues {
        //     log_likelihood: proposal_ll,
        //     ..
        // }) = proposal_database_entry;
        let forward_kernel_ll =
            distribution.kernel_log_likelihood(current, proposal);
        let backward_kernel_ll =
            distribution.kernel_log_likelihood(proposal, current);

        // The Metropoli-Hastings accept ratio
        let score = tracing_data.total_log_likelihood
            - previous_total_log_likelihood
            + backward_kernel_ll
            - forward_kernel_ll;

        if score > 0. || rand::random::<f64>().log2() < score {
            // println!("Accepting proposal!");
            tracing_data
                .trace
                .insert(wiggle_name, proposal_database_entry);
        } else {
            tracing_data.total_log_likelihood = previous_total_log_likelihood;
        }
    }

    results
}
