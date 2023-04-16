use std::fmt;

use crate::{
    bernoulli, normal,
    trace::{Trace, TraceEntry, TracePath},
    uniform,
};

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
pub enum ParametrizedValue {
    Bernoulli { value: bool, p: f64 },
    Uniform { value: f64, from: f64, to: f64 },
    Normal { value: f64, mean: f64, std_dev: f64 },
}

impl fmt::Display for ParametrizedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParametrizedValue::Bernoulli { value, p } => {
                write!(f, "bernoulli({}) => {}", p, value)
            }
            ParametrizedValue::Uniform { value, from, to } => {
                write!(f, "uniform({},{}) => {}", from, to, value)
            }
            ParametrizedValue::Normal {
                value,
                mean,
                std_dev,
            } => {
                write!(f, "normal({},{}) => {}", mean, std_dev, value)
            }
        }
    }
}

impl Sample<ParametrizedValue> {
    pub fn propose(&mut self) -> Proposal {
        let proposal = match &self.value {
            ParametrizedValue::Bernoulli { value, p } => {
                let dist = bernoulli(*p);
                dist.propose(value)
            }
            ParametrizedValue::Uniform { value, from, to } => {
                let dist = uniform(*from, *to);
                dist.propose(value)
            }
            ParametrizedValue::Normal {
                value,
                mean,
                std_dev,
            } => {
                let dist = normal(*mean, *std_dev);
                dist.propose(value)
            }
        };
        *self = proposal.sample.clone().into();
        proposal
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

impl<T: fmt::Display> fmt::Display for TracedSample<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.sample)?;
        writeln!(f, "{:?}", self.trace)?;
        Ok(())
    }
}

pub trait Distribution<_Tag, T> {
    fn sample(&self) -> TracedSample<T>;
    fn resample(&self, path: &TracePath, trace: &mut Trace) -> Sample<T>;
}

pub trait FnProb<T>: Fn(&TracePath, &mut Trace) -> Sample<T> {}

impl<T, F: Fn(&TracePath, &mut Trace) -> Sample<T>> FnProb<T> for F {}

pub enum _TagFnProb {}

impl<T, F: FnProb<T>> Distribution<_TagFnProb, T> for F {
    fn sample(&self) -> TracedSample<T> {
        let mut trace = Trace::new();
        let sample = self(&TracePath::new(), &mut trace);
        TracedSample { sample, trace }
    }

    fn resample(&self, path: &TracePath, trace: &mut Trace) -> Sample<T> {
        self(path, trace)
    }
}

pub trait PrimitiveDistribution<T: Clone> {
    fn raw_sample(&self) -> T;

    fn log_probability(&self, value: &T) -> f64;

    /// The `forward_log_probability` is P(proposal|prior) and the
    /// `reverse_log_probability` is P(prior|proposal). By default this is just
    /// resampling from the distribution independent of the prior.
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

    fn observe(&self, value: &T) -> f64 {
        self.log_probability(value)
    }
}

pub enum _TagPrimitiveDistribution {}

impl<T: Clone, D: PrimitiveDistribution<T>>
    Distribution<_TagPrimitiveDistribution, T> for D
{
    fn sample(&self) -> TracedSample<T> {
        let mut trace = Trace::new();
        let sample = self.resample(&TracePath::new(), &mut trace);
        TracedSample { sample, trace }
    }

    fn resample(&self, path: &TracePath, trace: &mut Trace) -> Sample<T> {
        if let Some(TraceEntry {
            sample, touched, ..
        }) = trace.get_mut(&path)
        {}

        todo!()
    }

    // fn resample(&self, trace: &mut Trace) -> Sample<T> {
    //     if let Some(TraceEntry { sample, touched }) = trace.pop() {
    //         if !touched {
    //             if let Some(Sample {
    //                 value,
    //                 log_probability,
    //             }) = self.deparametrize(sample.value)
    //             {
    //                 trace.push(
    //                     Sample {
    //                         value: self.parametrize(value.clone()),
    //                         log_probability,
    //                     }
    //                     .into(),
    //                 );
    //                 return Sample {
    //                     value,
    //                     log_probability,
    //                 };
    //             }
    //         }
    //     }

    //     let value = self.raw_sample();
    //     let log_probability = self.log_probability(&value);
    //     trace.push(
    //         Sample {
    //             value: self.parametrize(value.clone()),
    //             log_probability,
    //         }
    //         .into(),
    //     );
    //     Sample {
    //         value,
    //         log_probability,
    //     }
    // }
}
