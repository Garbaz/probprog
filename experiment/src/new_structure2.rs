use std::fmt;

use crate::trace::{ParametrizedValue, Trace};

#[derive(Debug, Clone)]
pub struct Sample<T> {
    pub value: T,
    pub log_likelihood: f64,
}

impl<T: fmt::Display> fmt::Display for Sample<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {:.3}", self.value, self.log_likelihood)
    }
}

#[derive(Debug, Clone)]
pub struct Proposal {
    pub sample: Sample<ParametrizedValue>,
    pub forward_log_likelihood: f64,
    pub backward_log_likelihood: f64,
}

#[derive(Debug, Clone)]
pub struct TracedSample<T> {
    pub sample: Sample<T>,
    pub trace: Trace,
}

pub trait Distribution<_Tag, T> {
    fn sample(&self) -> TracedSample<T>;
    fn resample(&self, trace: &mut Trace) -> Sample<T>;
}

pub trait FnProb<T>: Fn(&mut Trace) -> Sample<T> {}

impl<T, F: Fn(&mut Trace) -> Sample<T>> FnProb<T> for F {}

pub enum _TagFnProb {}

impl<T, F: FnProb<T>> Distribution<_TagFnProb, T> for F {
    fn sample(&self) -> TracedSample<T> {
        let mut trace = Trace::new();
        let sample = self(&mut trace);
        TracedSample { sample, trace }
    }

    fn resample(&self, trace: &mut Trace) -> Sample<T> {
        self(trace)
    }
}

pub trait PrimitiveDistribution<T: Clone> {
    fn raw_sample(&self) -> T;

    fn log_likelihood(&self, value: &T) -> f64;

    /// The `forward_log_likelihood` is P(sample|prior) and the
    /// `backward_log_likelihood` is P(prior|sample). By default this is just
    /// resampling from the distribution independent of the prior.
    fn propose(&self, prior: &T) -> Proposal {
        let value = self.raw_sample();
        let sample = Sample {
            log_likelihood: self.log_likelihood(&value),
            value: self.parametrized(value),
        };

        Proposal {
            forward_log_likelihood: sample.log_likelihood,
            sample,
            backward_log_likelihood: self.log_likelihood(prior),
        }
    }

    fn parametrized(&self, value: T) -> ParametrizedValue;

    fn observe(&self, trace: &mut Trace, value: T) -> f64 {
        let log_likelihood =self.log_likelihood(&value); 
        trace.push(
            Sample {
                value: self.parametrized(value),
                log_likelihood,
            }
            .into(),
        );
        log_likelihood
    }
}

pub enum _TagPrimitiveDistribution {}

impl<T: Clone, D: PrimitiveDistribution<T>>
    Distribution<_TagPrimitiveDistribution, T> for D
{
    fn sample(&self) -> TracedSample<T> {
        let mut trace = Trace::new();
        let sample = self.resample(&mut trace);
        TracedSample { sample, trace }
    }

    fn resample(&self, trace: &mut Trace) -> Sample<T> {
        let value = self.raw_sample();
        let log_likelihood = self.log_likelihood(&value);
        self.observe(trace, value.clone());
        Sample {
            value,
            log_likelihood,
        }
    }
}
