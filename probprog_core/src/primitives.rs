//! These are the probabilistic primitives which are to be used in
//! probabilistic functions. They can be used as ordinary functions anywhere,
//! but if used in a function annotated with `#[prob]`, they will be treated
//! specially to allow for efficient inference.
//! This means that the bodies of the functions here are only relevant if they
//! are used _outside_ a `prob` function.

use crate::{
    distribution::Distribution,
    distributions::{
        bernoulli::{Bernoulli, BernoulliParams},
        uniform::{Uniform, UniformParams},
    },
};

pub fn bernoulli(p: f64) -> bool {
    Bernoulli::new(BernoulliParams(p)).sample()
}

pub fn uniform(a: f64, b: f64) -> f64 {
    Uniform::new(UniformParams(a, b)).sample()
}
