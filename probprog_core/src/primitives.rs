//! These are the probabilistic primitives which are to be used in probabilistic
//! functions. They can be used as ordinary functions anywhere, but if used in a
//! function annotated with `#[prob]`, they will be treated specially to allow
//! for efficient inference. This means that the bodies of the functions here
//! are only relevant if they are used _outside_ a `prob` function. The primary
//! purpose for the functions here to exist is to ease the writing of
//! probabilistic functions in IDEs, i.e. for auto-complete & co

use crate::{
    __internal::probfunc::ProbFunc,
    __internal::{trace::{
        PrimitiveDistribution, PrimitiveSupportType, TraceEntry, TracingData,
        TracingPathRec,
    }, probfunc::ProbFn},
    distribution::Distribution,
    distributions::{
        bernoulli::{Bernoulli, BernoulliParams},
        uniform::{Uniform, UniformParams},
    },
};

pub fn bernoulli(
    p: f64,
) -> ProbFunc<bool, impl ProbFn<bool>> {
    ProbFunc::new(
        move |tracing_path: &mut TracingPathRec,
              tracing_data: &mut TracingData| {
            let params = BernoulliParams(p);
            let distribution =
                PrimitiveDistribution::Bernoulli(Bernoulli::new(params));

            match primitive_trace(distribution, tracing_path, tracing_data) {
                PrimitiveSupportType::Bernoulli(result) => Ok(result),
                _ => unreachable!(),
            }
        },
    )
}

pub fn uniform(
    a: f64,
    b: f64,
) -> ProbFunc<f64, impl ProbFn<f64>> {
    ProbFunc::new(
        move |tracing_path: &mut TracingPathRec,
              tracing_data: &mut TracingData| {
            let params = UniformParams(a, b);
            let distribution =
                PrimitiveDistribution::Uniform(Uniform::new(params));

            match primitive_trace(distribution, tracing_path, tracing_data) {
                PrimitiveSupportType::Uniform(result) => Ok(result),
                _ => unreachable!(),
            }
        },
    )
}

fn primitive_trace(
    distribution: PrimitiveDistribution,
    tracing_path: &mut TracingPathRec,
    tracing_data: &mut TracingData,
) -> PrimitiveSupportType {
    let tracing_path = tracing_path.next_variable();
    // println!("{:?} : {}", distribution.params(), tracing_path);

    let database_entry = match &tracing_data.proposal {
        // If there is a proposal, and it is for us, take it
        Some((n, entry)) if *n == tracing_path => Some(entry),
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
