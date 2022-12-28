use std::collections::BTreeMap;

use crate::{
    distribution::Distribution,
    distributions::{
        bernoulli::{Bernoulli, BernoulliParams},
        uniform::{Uniform, UniformParams},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TracingPath(String);

impl TracingPath {
    pub fn new() -> Self {
        TracingPath(String::new())
    }

    pub fn descend(&mut self, folder: &str) {
        self.0 += folder;
        self.0 += "/";
    }

    pub fn global_name(&self, local_name: &str) -> Self {
        Self(self.0.clone() + local_name)
    }
}

#[derive(Debug)]
pub struct TracingData {
    pub trace: BTreeMap<TracingPath, TraceEntry>,
    pub proposal: Option<(TracingPath, TraceEntry)>,
    pub trace_log_likelihood: f64,
}

impl TracingData {
    pub fn new() -> Self {
        Self {
            trace: BTreeMap::new(),
            proposal: None,
            trace_log_likelihood: 0.,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TraceEntry {
    pub distribution: PrimitiveDistribution,
    pub value: PrimitiveSupportType,
    pub log_likelihood: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveParamsType {
    Bernoulli(BernoulliParams),
    Uniform01(UniformParams),
}

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveSupportType {
    None,
    Bool(bool),
    F64(f64),
}

#[derive(Debug, Clone)]
pub enum PrimitiveDistribution {
    Bernoulli(Bernoulli),
    Uniform(Uniform),
}

impl PrimitiveDistribution {
    pub fn kind_eq(&self, other: &PrimitiveDistribution) -> bool {
        match (self, other) {
            (
                PrimitiveDistribution::Bernoulli(_),
                PrimitiveDistribution::Bernoulli(_),
            ) => true,
            (
                PrimitiveDistribution::Uniform(_),
                PrimitiveDistribution::Uniform(_),
            ) => true,
            _ => false,
        }
    }
}

impl Distribution for PrimitiveDistribution {
    type ParamsType = PrimitiveParamsType;
    type SupportType = PrimitiveSupportType;

    fn sample(&self) -> Self::SupportType {
        match self {
            PrimitiveDistribution::Bernoulli(d) => {
                PrimitiveSupportType::Bool(d.sample())
            }
            PrimitiveDistribution::Uniform(d) => {
                PrimitiveSupportType::F64(d.sample())
            }
        }
    }

    fn params(&self) -> Self::ParamsType {
        match self {
            PrimitiveDistribution::Bernoulli(d) => {
                PrimitiveParamsType::Bernoulli(d.params())
            }
            PrimitiveDistribution::Uniform(d) => {
                PrimitiveParamsType::Uniform01(d.params())
            }
        }
    }

    fn log_likelihood(&self, value: Self::SupportType) -> f64 {
        match (self, value) {
            (
                PrimitiveDistribution::Bernoulli(d),
                PrimitiveSupportType::Bool(value),
            ) => d.log_likelihood(value),
            (
                PrimitiveDistribution::Uniform(d),
                PrimitiveSupportType::F64(value),
            ) => d.log_likelihood(value),
            _ => f64::NAN,
        }
    }

    fn kernel_propose(&self, prior: Self::SupportType) -> Self::SupportType {
        match (self, prior) {
            (
                PrimitiveDistribution::Bernoulli(d),
                PrimitiveSupportType::Bool(prior),
            ) => PrimitiveSupportType::Bool(d.kernel_propose(prior)),
            (
                PrimitiveDistribution::Uniform(d),
                PrimitiveSupportType::F64(prior),
            ) => PrimitiveSupportType::F64(d.kernel_propose(prior)),
            _ => PrimitiveSupportType::None,
        }
    }

    fn kernel_log_likelihood(
        &self,
        prior: Self::SupportType,
        proposal: Self::SupportType,
    ) -> f64 {
        match (self, prior, proposal) {
            (
                PrimitiveDistribution::Bernoulli(d),
                PrimitiveSupportType::Bool(prior),
                PrimitiveSupportType::Bool(proposal),
            ) => d.kernel_log_likelihood(prior, proposal),
            (
                PrimitiveDistribution::Uniform(d),
                PrimitiveSupportType::F64(prior),
                PrimitiveSupportType::F64(proposal),
            ) => d.kernel_log_likelihood(prior, proposal),
            _ => f64::NAN,
        }
    }
}
