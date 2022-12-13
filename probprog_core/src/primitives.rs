//! These are the probabilistic primitives which are to be used in
//! probabilistic functions. They can be used as ordinary functions anywhere,
//! but if used in a function annotated with `#[prob], they will be treated
//! specially to allow for efficient inference.

use crate::{
    bernoulli::{Bernoulli, BernoulliParams},
    distribution::Distribution,
};

fn bernoulli(p: f64) -> bool {
    let d = Bernoulli::new(BernoulliParams { p }).unwrap();
    return d.sample();
}
