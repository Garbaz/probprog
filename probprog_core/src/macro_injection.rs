use crate::{
    distribution::Distribution,
    trace::{
        PrimitiveDistribution, PrimitiveSupportType, TraceEntry, TracingData,
        TracingPath,
    },
};

pub fn trace_macro_injection(
    distribution: PrimitiveDistribution,
    tracing_path: &TracingPath,
    tracing_data: &mut TracingData,
) -> PrimitiveSupportType {
    let database_entry = match &tracing_data.proposal {
        // If there is a proposal, and it is for us, take it
        Some((n, entry)) if n == tracing_path => Some(entry),
        // Otherwise, try looking in the trace for our entry
        _ => tracing_data.trace.get(&tracing_path),
    };
    match database_entry {
        Some(trace_entry)
            if trace_entry.distribution.params() == distribution.params() =>
        {
            // ^ The random choice in the database with our name has sampled
            // the same distribution with the same parameters.

            tracing_data.trace_log_likelihood += trace_entry.log_likelihood;

            trace_entry.value
        }
        Some(trace_entry)
            if trace_entry.distribution.kind_eq(&distribution) =>
        {
            // ^ The random choice in the database with our name has sampled
            // the same distribution, but with different parameters.
            // We reuse the value, but have to calculate a new log likelihood.

            let value = trace_entry.value;
            let log_likelihood = distribution.log_likelihood(value);
            tracing_data.trace.insert(
                tracing_path.clone(),
                TraceEntry {
                    distribution: distribution.clone(),
                    value,
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
                distribution: distribution.clone(),
                value,
                log_likelihood,
            };
            tracing_data.trace.insert(tracing_path.clone(), trace_entry);
            tracing_data.trace_log_likelihood += log_likelihood;
            value
        }
    }
}
