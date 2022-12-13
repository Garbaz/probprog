use rand::thread_rng;
use rand_distr as rd;

use crate::{distribution::Distribution, trace::TraceEntry};

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

impl Distribution for Bernoulli {
    type SupportType = bool;
    type ParamsType = BernoulliParams;

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

    fn likelihood(&self, value: Self::SupportType) -> f64 {
        match value {
            true => self.params.p,
            false => 1. - self.params.p,
        }
    }

    fn propose(&self, _current_value: Self::SupportType) -> Self::SupportType {
        self.sample()
    }

    fn proposal_likelihood(
        &self,
        current_value: Self::SupportType,
        proposal: Self::SupportType,
    ) -> f64 {
        self.likelihood(proposal)
    }
}
