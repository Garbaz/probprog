use std::collections::BTreeMap;

use crate::{bernoulli::Bernoulli, distribution::Distribution};

#[derive(Debug)]
pub enum TraceEntry {
    Bernoulli(
        <Bernoulli as Distribution>::ParamsType,
        <Bernoulli as Distribution>::SupportType,
    ),
}

impl TraceEntry {
    pub fn is_same_dist_as(&self, other: &Self) -> bool {
        match (self, other) {
            (
                TraceEntry::Bernoulli(params_self, _),
                TraceEntry::Bernoulli(params_other, _),
            ) => params_self == params_other,
        }
    }
}

#[derive(Debug)]
pub struct DatabaseEntry {
    pub trace_entry: TraceEntry,
    pub likelihood: f64,
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
