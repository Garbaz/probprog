//! An attempt to develop a better trace structure optimized for immutable
//! tracing.

use std::{collections::BTreeMap, ops::AddAssign};

use crate::new_structure2::Sample;

#[derive(Debug, Clone)]
pub enum ParametrizedValue {
    Bernoulli { value: bool, p: f64 },
    Uniform { value: f64, from: f64, to: f64 },
}

#[derive(Debug, Clone)]
pub struct TraceEntry {
    pub sample: Sample<ParametrizedValue>,
    pub touched: bool,
}

impl TraceEntry {
    pub fn new(value: ParametrizedValue, log_likelihood: f64) -> Self {
        Self {
            sample: Sample {
                value,
                log_likelihood,
            },
            touched: true,
        }
    }
}

impl From<Sample<ParametrizedValue>> for TraceEntry {
    fn from(sample: Sample<ParametrizedValue>) -> Self {
        Self {
            sample,
            touched: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TraceDirectory {
    Function(String),
    Recursion,
    Loop(usize),
}

pub type TracePath = Vec<TraceDirectory>;

#[derive(Debug, Clone)]
pub struct Trace {
    directories: BTreeMap<TraceDirectory, Trace>,
    variables: Vec<TraceEntry>,
}

impl Trace {
    pub fn new() -> Self {
        Self {
            directories: BTreeMap::new(),
            variables: Vec::new(),
        }
    }

    pub fn descend(&mut self, directory: TraceDirectory) -> &mut Trace {
        self.directories.entry(directory).or_insert(Trace::new())
    }

    pub fn attach(&mut self, directory: TraceDirectory, subtrace: Trace) {
        if let Some(t) = self.directories.get_mut(&directory) {
            *t += subtrace;
        } else {
            self.directories.insert(directory, subtrace);
        }
    }

    pub fn push(&mut self, trace_entry: TraceEntry) {
        self.variables.push(trace_entry);
    }

    pub fn push_at(
        &mut self,
        mut trace_path: TracePath,
        trace_entry: TraceEntry,
    ) {
        match trace_path.pop() {
            None => self.push(trace_entry),
            Some(dir) => {
                match self.directories.get_mut(&dir) {
                    Some(subtree) => {
                        subtree.push_at(trace_path, trace_entry);
                    }
                    None => {
                        let mut subtree = Trace::new();
                        subtree.push_at(trace_path, trace_entry);
                        self.directories.insert(dir, subtree);
                    }
                };
            }
        }
    }

    pub fn clean(&mut self) {
        self.variables.retain(|e| e.touched);
        for t in self.directories.values_mut() {
            for v in &mut t.variables {
                v.touched = false;
            }
            t.clean();
        }
    }
}

impl AddAssign for Trace {
    fn add_assign(&mut self, rhs: Self) {
        self.variables.extend(rhs.variables);
        for (d, t) in rhs.directories {
            self.attach(d, t);
        }
    }
}

impl From<TraceEntry> for Trace {
    fn from(trace_entry: TraceEntry) -> Self {
        let mut trace = Trace::new();
        trace.push(trace_entry);
        trace
    }
}

impl From<Sample<ParametrizedValue>> for Trace {
    fn from(sample: Sample<ParametrizedValue>) -> Self {
        TraceEntry::new(sample.value, sample.log_likelihood).into()
    }
}
