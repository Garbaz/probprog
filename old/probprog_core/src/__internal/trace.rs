use std::collections::BTreeMap;

use crate::{
    distribution::Distribution,
    distributions::{
        bernoulli::{Bernoulli, BernoulliParams},
        uniform::{Uniform, UniformParams},
    },
};

use super::tracingpath::TracingPath;

#[derive(Debug, Clone)]
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

    /// Remove all untouched entries in the trace, and reset `touched` for all.
    pub fn clean_trace(&mut self) {
        self.trace.retain(|_, e| e.touched);
        for e in self.trace.values_mut() {
            e.touched = false;
        }
    }
}

#[derive(Debug, Clone)]
pub struct TraceEntry {
    pub distribution: PrimitiveDistribution,
    pub value: PrimitiveSupportType,
    pub log_likelihood: f64,
    pub touched: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveParamsType {
    Bernoulli(BernoulliParams),
    Uniform(UniformParams),
}

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveSupportType {
    Bernoulli(bool),
    Uniform(f64),
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
                PrimitiveSupportType::Bernoulli(d.sample())
            }
            PrimitiveDistribution::Uniform(d) => {
                PrimitiveSupportType::Uniform(d.sample())
            }
        }
    }

    fn params(&self) -> Self::ParamsType {
        match self {
            PrimitiveDistribution::Bernoulli(d) => {
                PrimitiveParamsType::Bernoulli(d.params())
            }
            PrimitiveDistribution::Uniform(d) => {
                PrimitiveParamsType::Uniform(d.params())
            }
        }
    }

    fn log_likelihood(&self, value: Self::SupportType) -> f64 {
        match (self, value) {
            (
                PrimitiveDistribution::Bernoulli(d),
                PrimitiveSupportType::Bernoulli(value),
            ) => d.log_likelihood(value),
            (
                PrimitiveDistribution::Uniform(d),
                PrimitiveSupportType::Uniform(value),
            ) => d.log_likelihood(value),
            _ => unreachable!(),
        }
    }

    fn kernel_propose(&self, prior: Self::SupportType) -> Self::SupportType {
        match (self, prior) {
            (
                PrimitiveDistribution::Bernoulli(d),
                PrimitiveSupportType::Bernoulli(prior),
            ) => PrimitiveSupportType::Bernoulli(d.kernel_propose(prior)),
            (
                PrimitiveDistribution::Uniform(d),
                PrimitiveSupportType::Uniform(prior),
            ) => PrimitiveSupportType::Uniform(d.kernel_propose(prior)),
            _ => unreachable!(),
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
                PrimitiveSupportType::Bernoulli(prior),
                PrimitiveSupportType::Bernoulli(proposal),
            ) => d.kernel_log_likelihood(prior, proposal),
            (
                PrimitiveDistribution::Uniform(d),
                PrimitiveSupportType::Uniform(prior),
                PrimitiveSupportType::Uniform(proposal),
            ) => d.kernel_log_likelihood(prior, proposal),
            _ => unreachable!(),
        }
    }
}
