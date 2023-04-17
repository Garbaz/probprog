use crate::distribution::{PrimitiveDistribution, Sample};

use rand::thread_rng;
use rand_distr as rd;

use super::ParametrizedValue;

pub struct Uniform {
    pub dist: rd::Uniform<f64>,
    pub from: f64,
    pub to: f64,
}

impl Uniform {
    pub fn new(from: f64, to: f64) -> Self {
        Self {
            dist: rd::Uniform::new(from, to),
            from,
            to,
        }
    }
}

impl PrimitiveDistribution<f64> for Uniform {
    fn raw_sample(&self) -> f64 {
        rd::Distribution::sample(&self.dist, &mut thread_rng())
    }

    fn log_probability(&self, value: &f64) -> f64 {
        if &self.from < value && value <= &self.to {
            -((self.to - self.from).log2())
        } else {
            f64::NEG_INFINITY
        }
    }

    fn parametrize(&self, value: f64) -> ParametrizedValue {
        ParametrizedValue::Uniform {
            value,
            from: self.from,
            to: self.to,
        }
    }

    fn deparametrize(&self, value: ParametrizedValue) -> Option<Sample<f64>> {
        match value {
            ParametrizedValue::Uniform { value, .. } => Some(Sample {
                value,
                log_probability: self.log_probability(&value),
            }),
            _ => None,
        }
    }
}
