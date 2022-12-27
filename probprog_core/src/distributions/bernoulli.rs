use std::any::Any;

use rand::thread_rng;
use rand_distr as rd;

use crate::{
    distribution::Distribution, trace::{DistributionAndValue, PrimitiveDistributionAndValue},
    // trace::{TraceEntry, TraceEntryDistribution, TraceEntryValues},
};

#[derive(Clone, Debug)]
pub struct Bernoulli {
    pub dist: rd::Bernoulli,
    pub params: BernoulliParams,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BernoulliParams(pub f64);

impl Bernoulli {
    pub fn new(params: BernoulliParams) -> Self {
        Bernoulli {
            dist: rd::Bernoulli::new(params.0).unwrap(),
            params,
        }
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

    fn params(&self) -> Self::ParamsType {
        self.params
    }

    fn trace(&self, value: Self::SupportType) -> PrimitiveDistributionAndValue {
        PrimitiveDistributionAndValue::Bernoulli(DistributionAndValue { distribution: self.clone(), value })
    }

    fn log_likelihood(&self, value: Self::SupportType) -> f64 {
        match value {
            true => self.params.0.log2(),
            false => 1. - self.params.0.log2(),
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

    // fn to_trace_entry(&self, value: Self::SupportType) -> TraceEntry {
    //     TraceEntry::Bernoulli(TraceEntryValues::new(
    //         self.params,
    //         value,
    //         self.log_likelihood(value),
    //     ))
    // }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind_eq(&self, other: &impl Distribution) -> bool {
        other.as_any().downcast_ref::<Self>().is_some()
    }

    fn params_eq(&self, other: &impl Distribution) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self.params == other.params
        } else {
            false
        }
    }
}
