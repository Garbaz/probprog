use rand::thread_rng;
use rand_distr as rd;

use crate::distribution::Distribution;

#[derive(Clone, Debug)]
pub struct Uniform {
    pub dist: rd::Uniform<f64>,
    pub params: UniformParams,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UniformParams(pub f64, pub f64);

impl Uniform {
    pub fn new(params: UniformParams) -> Self {
        // Ensuring that a <= b
        let params =
            UniformParams(params.0.min(params.1), params.0.max(params.1));
        let a = params.0;
        let b = params.1;
        Uniform {
            dist: rd::Uniform::new(a, b),
            params,
        }
    }
}

impl Distribution for Uniform {
    type ParamsType = UniformParams;
    type SupportType = f64;

    fn sample(&self) -> Self::SupportType {
        rd::Distribution::sample(&self.dist, &mut thread_rng())
    }

    fn params(&self) -> Self::ParamsType {
        self.params
    }

    fn log_likelihood(&self, value: Self::SupportType) -> f64 {
        let a = self.params.0;
        let b = self.params.1;
        if a < value && value <= b {
            1. / (b - a)
            //^ this won't crash, since `a < x && x <= b` implies `a < b`
        } else {
            0.
        }
    }

    fn kernel_propose(&self, _prior: Self::SupportType) -> Self::SupportType {
        self.sample() // TODO: Is there a better proposal function?
    }

    fn kernel_log_likelihood(
        &self,
        _prior: Self::SupportType,
        proposal: Self::SupportType,
    ) -> f64 {
        self.log_likelihood(proposal)
    }
}
