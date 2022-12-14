use std::any::Any;

use crate::{trace::TraceEntry, bernoulli::Bernoulli};

// pub trait ParamsEq {
//     fn eq(&self, other : &Self) -> bool;
// }

pub trait DistributionEq : std::fmt::Debug {
    fn eq(&self, other : &impl DistributionEq) -> bool;
    fn params_eq(&self, other : &impl DistributionEq) -> bool;
    fn as_any(&self) -> &dyn Any;
}

pub trait Distribution : std::fmt::Debug {
    type SupportType;
    type ParamsType;
    type SelfComparable : DistributionEq;

    fn sample(&self) -> Self::SupportType;
    // fn support(&self) -> Self::SupportType;
    fn params(&self) -> Self::ParamsType;
    fn trace(&self, value: Self::SupportType) -> TraceEntry;

    fn log_likelihood(&self, value: Self::SupportType) -> f64;

    fn kernel_propose(&self, prior: Self::SupportType) -> Self::SupportType;
    fn kernel_log_likelihood(&self, prior : Self::SupportType, proposal : Self::SupportType) -> f64;

    fn as_comparable(&self) -> &Self::SelfComparable;
}