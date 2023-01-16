//! These are the probabilistic primitives which are to be used in probabilistic
//! functions. They can be used as ordinary functions anywhere, but if used in a
//! function annotated with `#[prob]`, they will be treated specially to allow
//! for efficient inference. This means that the bodies of the functions here
//! are only relevant if they are used _outside_ a `prob` function. The primary
//! purpose for the functions here to exist is to ease the writing of
//! probabilistic functions in IDEs, i.e. for auto-complete & co

use crate::{
    distributions::{
        bernoulli::{Bernoulli, BernoulliParams},
        uniform::{Uniform, UniformParams},
    },
    macro_injection::trace_macro_injection,
    probfunc::ProbFunc,
    trace::{
        PrimitiveDistribution, PrimitiveSupportType, TracingData, TracingPath,
    },
};

pub fn bernoulli(
    p: f64,
) -> ProbFunc<bool, impl Fn(TracingPath, &mut TracingData) -> bool> {
    ProbFunc(
        move |tracing_path: TracingPath, tracing_data: &mut TracingData| {
            let params = BernoulliParams(p);
            let distribution =
                PrimitiveDistribution::Bernoulli(Bernoulli::new(params));

            match trace_macro_injection(
                distribution,
                &tracing_path,
                tracing_data,
            ) {
                PrimitiveSupportType::Bernoulli(result) => result,
                _ => unreachable!(),
            }
        },
    )
}

pub fn uniform(
    a: f64,
    b: f64,
) -> ProbFunc<f64, impl Fn(TracingPath, &mut TracingData) -> f64> {
    ProbFunc(
        move |tracing_path: TracingPath, tracing_data: &mut TracingData| {
            let params = UniformParams(a, b);
            let distribution =
                PrimitiveDistribution::Uniform(Uniform::new(params));

            match trace_macro_injection(
                distribution,
                &tracing_path,
                tracing_data,
            ) {
                PrimitiveSupportType::Uniform(result) => result,
                _ => unreachable!(),
            }
        },
    )
}
