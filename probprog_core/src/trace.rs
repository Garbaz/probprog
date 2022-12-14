use std::collections::BTreeMap;

use crate::{
    bernoulli::Bernoulli,
    distribution::{Distribution, DistributionEq},
};

// BETTER:
// ("x", Bernoulli(0.5))
// (String, Box<dyn Distribution>)

#[derive(Debug)]
pub enum TraceEntry {
    /// TODO: We should probably structure this differently! All distributions have params and support.
    Bernoulli(
        <Bernoulli as Distribution>::ParamsType,
        <Bernoulli as Distribution>::SupportType,
    ),
}

// impl TraceEntry {
//     pub fn is_same_dist_as(&self, other: &Self) -> bool {
//         match (self, other) {
//             (
//                 TraceEntry::Bernoulli(params_self, _),
//                 TraceEntry::Bernoulli(params_other, _),
//             ) => params_self == params_other,
//         }
//     }

//     pub fn into_distribution(&self) -> impl Distribution {
//         match self {
//             TraceEntry::Bernoulli(params, _) => {
//                 Bernoulli::new(*params).unwrap()
//             }
//         }
//     }
// }

#[derive(Debug)]
pub struct TraceEntry2<ParamsType, SupportType, DistrType> {
    pub distribution: Box<
        dyn Distribution<
            ParamsType = ParamsType,
            SupportType = SupportType,
            SelfComparable = DistrType,
        >,
    >,
    pub value: SupportType,
    pub log_likelihood: f64,
}

#[derive(Debug)]
pub struct DatabaseEntry {
    pub trace_entry: TraceEntry,
    pub log_likelihood: f64,
}

#[derive(Debug)]
pub struct Database {
    tree: BTreeMap<String, DatabaseEntry>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        key: String,
        value: DatabaseEntry,
    ) -> Option<DatabaseEntry> {
        self.tree.insert(key, value)
    }

    pub fn get(&self, key: &String) -> Option<&DatabaseEntry> {
        self.tree.get(key)
    }
}
