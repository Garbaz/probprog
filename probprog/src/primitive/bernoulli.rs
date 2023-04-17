use crate::distribution::{PrimitiveDistribution, Sample};

use rand::thread_rng;
use rand_distr as rd;

use super::ParametrizedValue;

pub struct Bernoulli {
    pub dist: rd::Bernoulli,
    pub p: f64,
}

impl Bernoulli {
    pub fn new(p: f64) -> Self {
        Self {
            dist: rd::Bernoulli::new(p).unwrap(),
            p,
        }
    }
}

impl PrimitiveDistribution<bool> for Bernoulli {
    fn raw_sample(&self) -> bool {
        rd::Distribution::sample(&self.dist, &mut thread_rng())
    }
    fn log_probability(&self, value: &bool) -> f64 {
        match value {
            true => self.p.log2(),
            false => (1. - self.p).log2(),
        }
    }

    fn parametrize(&self, value: bool) -> ParametrizedValue {
        ParametrizedValue::Bernoulli { value, p: self.p }
    }

    fn deparametrize(&self, value: ParametrizedValue) -> Option<Sample<bool>> {
        match value {
            ParametrizedValue::Bernoulli { value, .. } => Some(Sample {
                value,
                log_probability: self.log_probability(&value),
            }),
            _ => None,
        }
    }
}
