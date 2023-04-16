pub use probprog_macro::*;

pub mod __internal;
pub mod distribution;
pub mod inference;
pub mod primitive;
pub mod trace;
pub mod trace3;
pub mod visualization;

use distribution::{Distribution, Sample};
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
) -> impl Iterator<Item = Sample<T>> {
    MetropolisHastings::new(distribution)
}

// pub fn draw_sample<_Tag, T: Clone, D: Distribution<_Tag, T>>(
//     distribution: D,
// ) -> TracedSample<T> {
//     distribution.sample()
// }
