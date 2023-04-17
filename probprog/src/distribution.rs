use std::fmt;

use crate::{primitive::ParametrizedValue, trace::Trace};

#[derive(Debug, Clone)]
pub struct Sample<T> {
    pub value: T,
    pub log_probability: f64,
}

impl<T: fmt::Display> fmt::Display for Sample<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {:.3}", self.value, self.log_probability.exp2())
    }
}

#[derive(Debug, Clone)]
pub struct Proposal {
    pub sample: Sample<ParametrizedValue>,
    /// P(proposed value | current value)
    pub forward_log_probability: f64,
    /// P(current value | proposed value)
    pub reverse_log_probability: f64,
}

#[derive(Debug, Clone)]
pub struct TracedSample<T> {
    pub sample: Sample<T>,
    pub trace: Trace,
}

pub trait Distribution<_Tag, T> {
    fn sample(&self) -> TracedSample<T>;
    fn resample(&self, trace: Trace) -> TracedSample<T>;
}

pub trait FnProb<T>: Fn(Trace) -> TracedSample<T> {}

impl<T, F: Fn(Trace) -> TracedSample<T>> FnProb<T> for F {}

pub enum _TagFnProb {}

impl<T, F: FnProb<T>> Distribution<_TagFnProb, T> for F {
    fn sample(&self) -> TracedSample<T> {
        self.resample(Trace::Empty)
    }

    fn resample(&self, trace: Trace) -> TracedSample<T> {
        self(trace)
    }
}

pub trait PrimitiveDistribution<T: Clone> {
    fn raw_sample(&self) -> T;

    fn log_probability(&self, value: &T) -> f64;

    /// By default, this just draws a new sample from the distribution
    /// independent of the prior, but an implementation can provide a more
    /// informed `propose` if possible.
    fn propose(&self, prior: &T) -> Proposal {
        let value = self.raw_sample();
        let sample = Sample {
            log_probability: self.log_probability(&value),
            value: self.parametrize(value),
        };

        Proposal {
            forward_log_probability: sample.log_probability,
            reverse_log_probability: self.log_probability(prior),
            sample,
        }
    }

    fn parametrize(&self, value: T) -> ParametrizedValue;
    fn deparametrize(&self, value: ParametrizedValue) -> Option<Sample<T>>;

    // fn observe(&self, value: &T) -> f64 {
    //     self.log_probability(value)
    // }
}

pub enum _TagPrimitiveDistribution {}

impl<T: Clone, D: PrimitiveDistribution<T>>
    Distribution<_TagPrimitiveDistribution, T> for D
{
    fn sample(&self) -> TracedSample<T> {
        self.resample(Trace::Empty)
    }

    fn resample(&self, trace: Trace) -> TracedSample<T> {
        let value = if let Trace::Primitive { sample } = trace {
            if let Some(Sample { value, .. }) = self.deparametrize(sample.value)
            {
                value
            } else {
                self.raw_sample()
            }
        } else {
            self.raw_sample()
        };

        let log_probability = self.log_probability(&value);
        let sample = Sample {
            value: value.clone(),
            log_probability,
        };
        let sample_pv = Sample {
            value: self.parametrize(value),
            log_probability,
        };
        TracedSample {
            sample,
            trace: Trace::Primitive { sample: sample_pv },
        }
    }
}
