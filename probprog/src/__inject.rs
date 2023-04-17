use crate::{
    distribution::{Distribution, PrimitiveDistribution},
    trace::{PushBackAndMut, Trace, Traces},
};

pub fn loop_descend<'a>(
    old_traces: &'a mut Traces,
    new_traces: &'a mut Traces,
    loop_counter: usize,
) -> (Traces, &'a mut Traces) {
    let old_traces = old_traces
        .pop_front()
        .unwrap_or_default()
        .loop_subtraces(loop_counter);
    let new_traces = new_traces
        .push_back_and_mut(Trace::Loop {
            iteration: loop_counter,
            subtraces: Traces::new(),
        })
        .subtraces()
        .unwrap();
    (old_traces, new_traces)
}

pub fn sample<T, _Tag, D: Distribution<_Tag, T>>(
    old_traces: &mut Traces,
    new_traces: &mut Traces,
    total_log_probability: &mut f64,
    distribution: D,
) -> T {
    let tsample =
        distribution.resample(old_traces.pop_front().unwrap_or_default());
    new_traces.push_back(tsample.trace);
    *total_log_probability += tsample.sample.log_probability;
    tsample.sample.value
}

pub fn observe<T: Clone, D: PrimitiveDistribution<T>>(
    total_log_probability: &mut f64,
    distribution: D,
    value: &T,
) {
    let log_probability = distribution.log_probability(value);

    *total_log_probability += log_probability;
}

pub fn condition<T: Clone, D: PrimitiveDistribution<T>>(
    total_log_probability: &mut f64,
    predicate: bool,
) {
    if predicate {
        *total_log_probability = f64::NEG_INFINITY;
    }
}
