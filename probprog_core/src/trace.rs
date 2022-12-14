use std::collections::BTreeMap;

use crate::{bernoulli::Bernoulli, distribution::Distribution};

#[derive(Debug)]
pub struct TraceValues<ParamsType, SupportType> {
    pub params: ParamsType,
    pub value: SupportType,
    pub log_likelihood: f64,
}

#[derive(Debug)]
pub enum TraceEntry {
    Bernoulli(
        TraceValues<
            <Bernoulli as Distribution>::ParamsType,
            <Bernoulli as Distribution>::SupportType,
        >,
    ),
}

#[derive(Debug)]
pub struct Trace {
    tree: BTreeMap<String, TraceEntry>,
}

impl Trace {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        key: String,
        value: TraceEntry,
    ) -> Option<TraceEntry> {
        self.tree.insert(key, value)
    }

    pub fn get(&self, key: &String) -> Option<&TraceEntry> {
        self.tree.get(key)
    }
}
