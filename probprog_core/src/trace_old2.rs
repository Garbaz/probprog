use std::{any::Any, collections::BTreeMap};

use crate::{
    distribution::Distribution,
    distributions::{bernoulli::Bernoulli, uniform::Uniform01},
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
pub enum PrimitiveDistributionAndValue {
    Bernoulli(DistributionAndValue<Bernoulli>),
    // Uniform01(DistributionAndValue<Uniform01>),
}

impl PrimitiveDistributionAndValue {
    pub fn distribution_and_value(&self) -> &impl DistributionWithValue {
        match self {
            PrimitiveDistributionAndValue::Bernoulli(dav) => dav,
            // PrimitiveDistributionAndValue::Uniform01(dav) => dav,
        }
        // if let PrimitiveDistributionAndValue::Bernoulli(dav) = self {
        //     return dav.clone();
        // }
        // if let PrimitiveDistributionAndValue::Uniform01(dav) = self {
        //     return dav.clone();
        // }
        // unreachable!()
    }
}

pub trait DistributionWithValue: Distribution {
    fn value(&self) -> Self::SupportType;
}

#[derive(Debug, Clone)]
pub struct DistributionAndValue<D: Distribution> {
    pub distribution: D,
    pub value: D::SupportType,
}

impl<D: Distribution + 'static> Distribution for DistributionAndValue<D> {
    type ParamsType = D::ParamsType;

    type SupportType = D::SupportType;

    fn sample(&self) -> Self::SupportType {
        self.distribution.sample()
    }

    fn params(&self) -> Self::ParamsType {
        self.distribution.params()
    }

    fn trace(&self, value: Self::SupportType) -> PrimitiveDistributionAndValue {
        self.distribution.trace(value)
    }

    fn log_likelihood(&self, value: Self::SupportType) -> f64 {
        self.distribution.log_likelihood(value)
    }

    fn kernel_propose(&self, prior: Self::SupportType) -> Self::SupportType {
        self.distribution.kernel_propose(prior)
    }

    fn kernel_log_likelihood(
        &self,
        prior: Self::SupportType,
        proposal: Self::SupportType,
    ) -> f64 {
        self.distribution.kernel_log_likelihood(prior, proposal)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn kind_eq(&self, other: &impl Distribution) -> bool {
        self.distribution.kind_eq(other)
    }

    fn params_eq(&self, other: &impl Distribution) -> bool {
        self.distribution.params_eq(other)
    }
}

impl<D: Distribution + 'static> DistributionWithValue
    for DistributionAndValue<D>
{
    fn value(&self) -> Self::SupportType {
        self.value
    }
}

#[derive(Debug, Clone)]
pub struct TraceEntry {
    pub distribution_and_value: PrimitiveDistributionAndValue,
    pub log_likelihood: f64,
}
