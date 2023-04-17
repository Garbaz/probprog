use std::fmt;

use crate::distribution::{PrimitiveDistribution, Proposal, Sample};

use super::{Bernoulli, Normal, Uniform};

#[derive(Debug, Clone)]
pub enum ParametrizedValue {
    Bernoulli { value: bool, p: f64 },
    Uniform { value: f64, from: f64, to: f64 },
    Normal { value: f64, mean: f64, std_dev: f64 },
}

impl ParametrizedValue {
    pub fn value_eq(&self, other: &Self) -> bool {
        use ParametrizedValue::*;
        match (self, other) {
            (Bernoulli { value, .. }, Bernoulli { value: value_, .. }) => {
                value == value_
            }
            (Uniform { value, .. }, Uniform { value: value_, .. }) => {
                value == value_
            }
            (Normal { value, .. }, Normal { value: value_, .. }) => {
                value == value_
            }
            _ => false,
        }
    }
}

impl Sample<ParametrizedValue> {
    pub fn propose(&mut self) -> Proposal {
        let proposal = match &self.value {
            ParametrizedValue::Bernoulli { value, p } => {
                let dist = Bernoulli::new(*p);
                dist.propose(value)
            }
            ParametrizedValue::Uniform { value, from, to } => {
                let dist = Uniform::new(*from, *to);
                dist.propose(value)
            }
            ParametrizedValue::Normal {
                value,
                mean,
                std_dev,
            } => {
                let dist = Normal::new(*mean, *std_dev);
                dist.propose(value)
            }
        };
        *self = proposal.sample.clone();
        proposal
    }
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
