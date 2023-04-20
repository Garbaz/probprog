pub use probprog_macro::*;

pub mod __inject;
pub mod distribution;
pub mod inference;
pub mod primitive;
pub mod trace;
pub mod visualization;

use distribution::Distribution;
use inference::MetropolisHastings;
use primitive::{Bernoulli, Normal, Uniform};

pub fn bernoulli(p: f64) -> Bernoulli {
    Bernoulli::new(p)
}

pub fn uniform(from: f64, to: f64) -> Uniform {
    Uniform::new(from, to)
}

pub fn normal(mean: f64, std_dev: f64) -> Normal {
    Normal::new(mean, std_dev)
}

pub fn inference<_Tag, T: Clone, D: Distribution<_Tag, T>>(
    distribution: D,
) -> impl Iterator<Item = T> {
    MetropolisHastings::new(distribution)
}