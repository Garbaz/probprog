use crate::distribution::{PrimitiveDistribution, Sample};

use rand::thread_rng;
use rand_distr as rd;

use super::ParametrizedValue;

pub struct Normal {
    dist: rd::Normal<f64>,
    mean: f64,
    std_dev: f64,
}

impl Normal {
    pub fn new(mean: f64, std_dev: f64) -> Self {
        Self {
            dist: rd::Normal::new(mean, std_dev).unwrap(),
            mean,
            std_dev,
        }
    }
}

impl PrimitiveDistribution<f64> for Normal {
    fn raw_sample(&self) -> f64 {
        rd::Distribution::sample(&self.dist, &mut thread_rng())
    }

    fn log_probability(&self, value: &f64) -> f64 {
        const LOG_TWO_PI_HALF: f64 = 1.3257480647361595;
        let vn = (value - self.mean) / self.std_dev;
        let a = -0.5 * std::f64::consts::LOG2_E * (vn * vn);
        let b = self.std_dev.log2() + LOG_TWO_PI_HALF;
        a - b
    }

    fn parametrize(&self, value: f64) -> ParametrizedValue {
        ParametrizedValue::Normal {
            value,
            mean: self.mean,
            std_dev: self.std_dev,
        }
    }

    fn deparametrize(&self, value: ParametrizedValue) -> Option<Sample<f64>> {
        match value {
            ParametrizedValue::Normal { value, .. } => Some(Sample {
                value,
                log_probability: self.log_probability(&value),
            }),
            _ => None,
        }
    }
}
