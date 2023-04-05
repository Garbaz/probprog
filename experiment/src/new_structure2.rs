use crate::trace::{ParametrizedValue, TracedSample};
use rand::thread_rng;
use rand_distr as rd;

pub trait FnProb<T>: Fn() -> TracedSample<T> {}

impl<T, F: Fn() -> TracedSample<T>> FnProb<T> for F {}

#[derive(Debug, Clone)]
pub struct Sample<T> {
    pub value: T,
    pub log_likelihood: f64,
}

pub trait PrimitiveDistribution<T: Clone> {
    fn _raw_sample(&self) -> T;
    fn sample(&self) -> Sample<T> {
        let value = self._raw_sample();
        Sample {
            log_likelihood: self.log_probability_density(&value),
            value,
        }
    }
    fn log_probability_density(&self, value: &T) -> f64;
    fn kernel_propose(&self, prior: &T) -> Sample<T>;
    fn parametrized_value(&self, value: T) -> ParametrizedValue;
    fn parametrized_sample(
        &self,
        sample: Sample<T>,
    ) -> Sample<ParametrizedValue> {
        Sample {
            value: self.parametrized_value(sample.value),
            log_likelihood: sample.log_likelihood,
        }
    }
    fn sample_traced(&self) -> TracedSample<T> {
        self.observe_traced(self._raw_sample())
    }
    fn observe_traced(&self, value: T) -> TracedSample<T> {
        let sample = Sample {
            log_likelihood: self.log_probability_density(&value),
            value,
        };
        let trace = self.parametrized_sample(sample.clone()).into();
        TracedSample { sample, trace }
    }
}

pub struct Bernoulli {
    pub dist: rd::Bernoulli,
    pub p: f64,
}

impl PrimitiveDistribution<bool> for Bernoulli {
    fn _raw_sample(&self) -> bool {
        rd::Distribution::sample(&self.dist, &mut thread_rng())
    }
    fn log_probability_density(&self, value: &bool) -> f64 {
        match value {
            true => self.p.log2(),
            false => (1. - self.p).log2(),
        }
    }

    fn kernel_propose(&self, _prior: &bool) -> Sample<bool> {
        self.sample()
    }

    fn parametrized_value(&self, value: bool) -> ParametrizedValue {
        ParametrizedValue::Bernoulli { value, p: self.p }
    }
}

pub struct Uniform {
    pub dist: rd::Uniform<f64>,
    pub from: f64,
    pub to: f64,
}

impl PrimitiveDistribution<f64> for Uniform {
    fn _raw_sample(&self) -> f64 {
        rd::Distribution::sample(&self.dist, &mut thread_rng())
    }

    fn log_probability_density(&self, value: &f64) -> f64 {
        if &self.from < value && value <= &self.to {
            -((self.to - self.from).log2())
        } else {
            f64::NEG_INFINITY
        }
    }

    fn kernel_propose(&self, _prior: &f64) -> Sample<f64> {
        self.sample()
    }

    fn parametrized_value(&self, value: f64) -> ParametrizedValue {
        ParametrizedValue::Uniform {
            value,
            from: self.from,
            to: self.to,
        }
    }
}

pub fn bernoulli(p: f64) -> Bernoulli {
    Bernoulli {
        dist: rd::Bernoulli::new(p).unwrap(),
        p,
    }
}

pub fn uniform(from: f64, to: f64) -> Uniform {
    Uniform {
        dist: rd::Uniform::new(from, to),
        from,
        to,
    }
}

pub mod playground {
    use super::{Sample, uniform, PrimitiveDistribution};
    use crate::trace::{Trace, TraceDirectory, TracedSample};

    /* TODO */
    // fn sum_uniform() -> TracedSample<f64> {
    //     let __trace_directory = TraceDirectory::Function("sum_uniform".to_string());

    //     let x = uniform(-1., 1.).sample_traced();

    // }
}
