use std::any::Any;

use crate::trace::PrimitiveDistributionAndValue;

// use crate::trace::{TraceEntry, TraceEntryDistribution};

pub trait Distribution : Clone{
    type ParamsType;
    type SupportType : Copy;

    fn sample(&self) -> Self::SupportType;
    fn params(&self) -> Self::ParamsType;

    fn trace(&self, value: Self::SupportType) -> PrimitiveDistributionAndValue;

    fn log_likelihood(&self, value: Self::SupportType) -> f64;

    fn kernel_propose(&self, prior: Self::SupportType) -> Self::SupportType;
    fn kernel_log_likelihood(
        &self,
        prior: Self::SupportType,
        proposal: Self::SupportType,
    ) -> f64;

    // fn to_trace_entry(&self, value: Self::SupportType) -> TraceEntry;
    fn as_any(&self) -> &dyn Any;
    fn kind_eq(&self, other: &impl Distribution) -> bool;
    fn params_eq(&self, other: &impl Distribution) -> bool;
}