use std::fmt;

use crate::trace::{ParametrizedValue, Trace, TraceEntry};

#[derive(Debug, Clone)]
pub struct Sample<T> {
    pub value: T,
    pub log_likelihood: f64,
}

impl<T: fmt::Display> fmt::Display for Sample<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {:.3}", self.value, self.log_likelihood.exp2())
    }
}

#[derive(Debug, Clone)]
pub struct Proposal {
    pub sample: Sample<ParametrizedValue>,
    /// P(new value | current value)
    pub forward_log_likelihood: f64,
    /// P(current value | new value)
    pub backward_log_likelihood: f64,
}

#[derive(Debug, Clone)]
pub struct TracedSample<T> {
    pub sample: Sample<T>,
    pub trace: Trace,
}

impl<T: fmt::Display> fmt::Display for TracedSample<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.sample)?;
        writeln!(f, "{}", self.trace)
    }
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
            value: self.parametrize(value),
        };

        Proposal {
            forward_log_likelihood: sample.log_likelihood,
            sample,
            backward_log_likelihood: self.log_likelihood(prior),
        }
    }

    fn parametrize(&self, value: T) -> ParametrizedValue;
    fn deparametrize(&self, value: ParametrizedValue) -> Option<Sample<T>>;

    fn observe(&self, value: &T) -> f64 {
        self.log_likelihood(value)
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
        if let Some(TraceEntry { sample, touched }) = trace.pop() {
            if !touched {
                if let Some(Sample {
                    value,
                    log_likelihood,
                }) = self.deparametrize(sample.value)
                {
                    trace.push(
                        Sample {
                            value: self.parametrize(value.clone()),
                            log_likelihood,
                        }
                        .into(),
                    );
                    return Sample {
                        value,
                        log_likelihood,
                    };
                }
            }
        }

        let value = self.raw_sample();
        let log_likelihood = self.log_likelihood(&value);
        trace.push(
            Sample {
                value: self.parametrize(value.clone()),
                log_likelihood,
            }
            .into(),
        );
        Sample {
            value,
            log_likelihood,
        }
    }
}
