use std::collections::BTreeMap;

use crate::{
    bernoulli::{Bernoulli, BernoulliParams},
    distribution::{Distribution},
};

#[derive(Debug)]
pub struct TraceEntry {
    pub trace_entry_distribution: TraceEntryDistribution,
    pub log_likelihood: f64,
}

#[derive(Debug)]
pub enum TraceEntryDistribution {
    Bernoulli(
        TraceEntryValues<
            <Bernoulli as Distribution>::ParamsType,
            <Bernoulli as Distribution>::SupportType,
        >,
    ),
}

#[derive(Debug)]
pub struct TraceEntryValues<ParamsType, SupportType>(
    pub ParamsType,
    pub SupportType,
);

pub struct DistributionWithValue<D: Distribution>(D, D::SupportType);

impl<D : Distribution> Distribution for DistributionWithValue<D> {
    type ParamsType = D::ParamsType;

    type SupportType = D::SupportType;

    fn sample(&self) -> Self::SupportType {
        self.0.sample()
    }

    fn params(&self) -> Self::ParamsType {
        self.0.params()
    }

    fn trace(&self, value: Self::SupportType) -> TraceEntryDistribution {
        self.0.trace(value)
    }

    fn log_likelihood(&self, value: &Self::SupportType) -> f64 {
        self.0.log_likelihood(value)
    }

    fn kernel_propose(&self, prior: &Self::SupportType) -> Self::SupportType {
        self.0.kernel_propose(prior)
    }

    fn kernel_log_likelihood(
        &self,
        prior: &Self::SupportType,
        proposal: &Self::SupportType,
    ) -> f64 {
        self.0.kernel_log_likelihood(prior, proposal)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self.0.as_any()
    }

    fn kind_eq(&self, other: &impl Distribution) -> bool {
        self.0.kind_eq(other)
    }

    fn params_eq(&self, other: &impl Distribution) -> bool {
        self.0.params_eq(other)
    }
}

impl<D: Distribution> DistributionAndValue for DistributionWithValue<D> {
    fn value(&self) -> &Self::SupportType {
        &self.1
    }
}

pub trait Bob {
    type ParamsType;
    type SupportType;
    fn params(&self) -> &Self::ParamsType;
    fn value(&self) -> &Self::SupportType;
}

impl<ParamsType, SupportType> Bob
    for TraceEntryValues<ParamsType, SupportType>
{
    type ParamsType = ParamsType;
    type SupportType = SupportType;

    fn params(&self) -> &Self::ParamsType {
        &self.0
    }

    fn value(&self) -> &Self::SupportType {
        &self.1
    }
}

impl TraceEntryDistribution {
    pub fn to_distribution(&self) -> impl Distribution {
        match self {
            &TraceEntryDistribution::Bernoulli(TraceEntryValues(params, _)) => {
                Bernoulli::new(params).unwrap()
            }
        }
    }

    pub fn as_trace_entry_values<ParamsType, SupportType>(&self) -> &impl Bob {
        match self {
            TraceEntryDistribution::Bernoulli(trace_entry_values) => {
                trace_entry_values
            }
        }
    }
}

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

// #[derive(Debug)]
// pub struct TraceEntryDistribution<ParamsType, SupportType> {
//     pub params: ParamsType,
//     pub value: SupportType,
//     // pub log_likelihood: f64,
// }

// #[derive(Debug)]
// pub struct TraceEntryValues<ParamsType, SupportType> {
//     pub trace_entry_distribution:
//         TraceEntryDistribution<ParamsType, SupportType>,
//     pub log_likelihood: f64,
// }

// impl<ParamsType, SupportType> TraceEntryValues<ParamsType, SupportType> {
//     pub fn new(
//         params: ParamsType,
//         value: SupportType,
//         log_likelihood: f64,
//     ) -> Self {
//         TraceEntryValues {
//             trace_entry_distribution: TraceEntryDistribution { params, value },
//             log_likelihood,
//         }
//     }
// }

// #[derive(Debug)]
// pub enum TraceEntry {
//     Bernoulli(
//         TraceEntryValues<
//             <Bernoulli as Distribution>::ParamsType,
//             <Bernoulli as Distribution>::SupportType,
//         >,
//     ),
// }

// impl TraceEntry {
//     pub fn from_distribution<ParamsType, SupportType>(
//         distribution: &impl Distribution<
//             ParamsType = ParamsType,
//             SupportType = SupportType,
//         >,
//         value: SupportType,
//     ) -> Self {
//         distribution.trace(value)
//     }

//     pub fn to_distribution(&self) -> impl Distribution {
//         match self {
//             &TraceEntry::Bernoulli(TraceEntryValues {
//                 trace_entry_distribution: TraceEntryDistribution { params, .. },
//                 ..
//             }) => Bernoulli::new(params).unwrap(),
//         }
//     }
// }

// pub struct TraceEntry2<ParamsType, SupportType> {
//     pub dist:
//         dyn Distribution<ParamsType = ParamsType, SupportType = SupportType>,
//     pub value: SupportType,
//     pub log_likelihood: f64,
// }

// pub struct Trace2 {
//     tree: BTreeMap<String, TraceEntry2<_, _>>,
// }

// #[derive(Debug)]
// pub struct Trace {
//     tree: BTreeMap<String, TraceEntry>,
// }

// impl Trace {
//     pub fn new() -> Self {
//         Self {
//             tree: BTreeMap::new(),
//         }
//     }

//     pub fn insert(
//         &mut self,
//         key: String,
//         value: TraceEntry,
//     ) -> Option<TraceEntry> {
//         self.tree.insert(key, value)
//     }

//     pub fn get(&self, key: &String) -> Option<&TraceEntry> {
//         self.tree.get(key)
//     }
// }
