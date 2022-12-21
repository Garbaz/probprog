use rand::thread_rng;
use rand_distr as rd;

use crate::{
    distribution::Distribution,
    trace::{TraceEntry, TraceEntryValues},
};

#[derive(Clone, Debug)]
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

// impl DistributionCmp for Bernoulli {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }

//     fn type_eq(&self, other: &(dyn DistributionCmp)) -> bool {
//         other.as_any().downcast_ref::<Self>().is_some()
//     }

//     fn params_eq(&self, other: &(dyn DistributionCmp)) -> bool {
//         if let Some(other) = other.as_any().downcast_ref::<Self>() {
//             self.params == other.params
//         } else {
//             false
//         }
//     }
// }

impl Distribution for Bernoulli {
    type ParamsType = BernoulliParams;
    type SupportType = bool;

    fn sample(&self) -> Self::SupportType {
        rd::Distribution::sample(&self.dist, &mut thread_rng())
    }

    // fn support(&self) -> Self::SupportType {
    //     todo!()
    // }

    fn params(&self) -> Self::ParamsType {
        self.params
    }

    fn trace(
        &self,
        value: Self::SupportType,
        log_likelihood: f64,
    ) -> TraceEntry {
        TraceEntry::Bernoulli(TraceEntryValues {
            params: self.params,
            value,
            log_likelihood,
        })
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
}
