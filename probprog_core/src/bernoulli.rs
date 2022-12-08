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

    fn trace(&self, current_value: Self::SupportType) -> TraceEntry {
        TraceEntry::Bernoulli(self.params(), current_value)
    }

    fn propose(
        &self,
        current_value: Self::SupportType,
    ) -> (f64, Self::SupportType) {
        let r = self.sample();
        let p = match r {
            true => self.params.p,
            false => 1. - self.params.p,
        };
        (p, r)
    }
}
