use rand::thread_rng;
use rand_distr as rd;

use crate::{
    distribution::{DistributionEq, Distribution},
    trace::TraceEntry,
};

#[derive(Clone,Debug)]
pub struct Bernoulli {
    pub dist: rd::Bernoulli,
    pub params: BernoulliParams,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BernoulliParams {
    pub p: f64,
}

impl Bernoulli {
    pub fn new(params: BernoulliParams) -> Result<Self, rd::BernoulliError> {
        Ok(Bernoulli {
            dist: rd::Bernoulli::new(params.p)?,
            params,
        })
    }
}

impl DistributionEq for Bernoulli {
    fn eq(&self, other: &impl DistributionEq) -> bool {
        other.as_any().downcast_ref::<Bernoulli>().is_some()
    }

    fn params_eq(&self, other : &impl DistributionEq) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Bernoulli>() {
            other.params.p == self.params.p
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Distribution for Bernoulli {
    type SupportType = bool;
    type ParamsType = BernoulliParams;
    type SelfComparable = Self;

    fn sample(&self) -> Self::SupportType {
        rd::Distribution::sample(&self.dist, &mut thread_rng())
    }

    // fn support(&self) -> Self::SupportType {
    //     todo!()
    // }

    fn params(&self) -> Self::ParamsType {
        self.params
    }

    fn trace(&self, value: Self::SupportType) -> TraceEntry {
        TraceEntry::Bernoulli(self.params(), value)
    }

    fn log_likelihood(&self, value: Self::SupportType) -> f64 {
        match value {
            true => self.params.p.log2(),
            false => 1. - self.params.p.log2(),
        }
    }

    fn kernel_propose(&self, _prior: Self::SupportType) -> Self::SupportType {
        self.sample()
    }

    fn kernel_log_likelihood(
        &self,
        _prior: Self::SupportType,
        proposal: Self::SupportType,
    ) -> f64 {
        self.log_likelihood(proposal)
    }

    fn as_comparable(&self) -> &Self::SelfComparable {
        self
    }
}
