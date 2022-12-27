use std::any::Any;

use rand::thread_rng;
use rand_distr as rd;

use crate::{
    distribution::Distribution, trace::{DistributionAndValue, PrimitiveDistributionAndValue},
    // trace::{TraceEntry, TraceEntryDistribution, TraceEntryValues},
};

#[derive(Clone, Debug)]
pub struct Uniform01 {
    pub dist: rd::OpenClosed01,
    pub params: UniformParams,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UniformParams();

impl Uniform01 {
    pub fn new(params: UniformParams) -> Self {
        Uniform01 {
            dist: rd::OpenClosed01,
            params,
        }
    }
}

// impl DistributionCmp for Uniform {
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

impl Distribution for Uniform01 {
    type ParamsType = UniformParams;
    type SupportType = f64;

    fn sample(&self) -> Self::SupportType {
        rd::Distribution::sample(&self.dist, &mut thread_rng())
    }

    fn params(&self) -> Self::ParamsType {
        self.params
    }

    fn trace(&self, value: Self::SupportType) -> PrimitiveDistributionAndValue {
        // PrimitiveDistributionAndValue::Uniform01(DistributionAndValue { distribution: self.clone(), value })
        todo!()
    }

    fn log_likelihood(&self, value: Self::SupportType) -> f64 {
        // match value {
        //     true => self.params.0.log2(),
        //     false => 1. - self.params.0.log2(),
        // }
        todo!("The log likelihood of uniform...")
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
    //     TraceEntry::Uniform(TraceEntryValues::new(
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
