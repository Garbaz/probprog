use crate::{
    distribution::{Distribution, PrimitiveDistribution},
    trace::Trace,
};

pub fn sample<T, _Tag, D: Distribution<_Tag, T>>(
    trace: &mut Trace,
    log_probability: &mut f64,
    distribution: D,
) -> T {
    let sample = distribution.resample(trace);
    *log_probability += sample.log_probability;
    sample.value
}

pub fn observe<T: Clone, D: PrimitiveDistribution<T>>(
    /* trace: &mut Trace, */
    log_probability: &mut f64,
    distribution: D,
    value: &T,
) {
    *log_probability += distribution.observe(value);
}

pub fn condition<T: Clone, D: PrimitiveDistribution<T>>(
    log_probability: &mut f64,
    predicate: bool,
) {
    if !predicate {
        *log_probability = f64::NEG_INFINITY
    }
}
