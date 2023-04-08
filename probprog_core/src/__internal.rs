use crate::{
    distribution::{Distribution, PrimitiveDistribution},
    trace::Trace,
};

pub fn sample<T, _Tag, D: Distribution<_Tag, T>>(
    trace: &mut Trace,
    log_likelihood: &mut f64,
    distribution: D,
) -> T {
    let s = distribution.resample(trace);
    *log_likelihood += s.log_likelihood;
    s.value
}

pub fn observe<T: Clone, D: PrimitiveDistribution<T>>(
    /* trace: &mut Trace, */
    log_likelihood: &mut f64,
    distribution: D,
    value: &T,
) {
    *log_likelihood += distribution.observe(value);
}

pub fn condition<T: Clone, D: PrimitiveDistribution<T>>(
    log_likelihood: &mut f64,
    predicate: bool,
) {
    if ! predicate {
        *log_likelihood = f64::NEG_INFINITY
    }
}
