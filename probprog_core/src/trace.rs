use std::{
    collections::{BTreeMap, VecDeque},
    fmt,
    ops::AddAssign,
};

use rand::{thread_rng, Rng};

use crate::{
    bernoulli,
    distribution::{PrimitiveDistribution, Proposal, Sample},
    normal, uniform,
};

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

#[derive(Debug, Clone)]
pub struct TraceEntry {
    pub sample: Sample<ParametrizedValue>,
    pub touched: bool,
}

impl TraceEntry {
    pub fn propose(&mut self) -> Proposal {
        let proposal = match &self.sample.value {
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
impl fmt::Display for TraceEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.sample)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TraceDirectory {
    Function(String),
    Loop(usize),
    // Recursion,
}

impl fmt::Display for TraceDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TraceDirectory::Function(n) => write!(f, "{}", n),
            // TraceDirectory::Recursion => write!(f, "."),
            TraceDirectory::Loop(c) => write!(f, "@{}", c),
        }
    }
}

pub type TracePath = Vec<TraceDirectory>;

#[derive(Debug, Clone)]
pub struct Trace {
    pub directories: BTreeMap<TraceDirectory, Trace>,
    pub variables: VecDeque<TraceEntry>,
}

impl Trace {
    pub fn new() -> Self {
        Self {
            directories: BTreeMap::new(),
            variables: VecDeque::new(),
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

    pub fn pop(&mut self) -> Option<TraceEntry> {
        self.variables.pop_front()
    }

    pub fn push(&mut self, trace_entry: TraceEntry) {
        self.variables.push_back(trace_entry);
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
        for v in &mut self.variables {
            v.touched = false;
        }

        for t in self.directories.values_mut() {
            t.clean();
        }
        self.directories.retain(|_, t| {
            !(t.variables.is_empty() && t.directories.is_empty())
        });
    }

    pub fn random_variable(&mut self) -> Option<&mut TraceEntry> {
        // This could be more efficiently if we add some stuff, like for a first
        // step a counter for the total number of variables below a certain
        // trace node, but this should be good enough for now.

        let mut vars: Vec<_> = self.iter_mut().collect();
        if vars.is_empty() {
            None
        } else {
            let n = thread_rng().gen_range(0..vars.len());
            Some(vars.swap_remove(n))
        }
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a TraceEntry> + 'a> {
        Box::new(
            self.variables
                .iter()
                .chain(self.directories.iter().flat_map(|(_, t)| t.iter())),
        )
    }

    pub fn iter_mut<'a>(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = &'a mut TraceEntry> + 'a> {
        Box::new(
            self.variables.iter_mut().chain(
                self.directories.iter_mut().flat_map(|(_, t)| t.iter_mut()),
            ),
        )
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

impl IntoIterator for Trace {
    type Item = TraceEntry;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            self.variables.into_iter().chain(
                self.directories
                    .into_iter()
                    .flat_map(|(_, t)| t.into_iter()),
            ),
        )
    }
}

impl<'a> IntoIterator for &'a Trace {
    type Item = &'a TraceEntry;

    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Trace {
    type Item = &'a mut TraceEntry;

    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
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

impl fmt::Display for Trace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "│")?;
        {
            let mut iter = self.variables.iter();
            let mut next = iter.next();
            while let Some(v) = next {
                next = iter.next();
                if next.is_some() || !self.directories.is_empty() {
                    writeln!(f, "├─ {}", v)?;
                } else {
                    writeln!(f, "╰─ {}", v)?;
                }
            }
        }
        {
            let mut iter = self.directories.iter();
            let mut next = iter.next();
            while let Some((d, t)) = next {
                next = iter.next();
                writeln!(f, "│")?;
                if next.is_some() {
                    writeln!(f, "├── {}", d)?;
                } else {
                    writeln!(f, "╰── {}", d)?;
                }
                let ts = format!("{}", t);
                for l in ts.lines() {
                    if next.is_some() {
                        writeln!(f, "│   {}", l)?;
                    } else {
                        writeln!(f, "    {}", l)?;
                    }
                }
            }
        }
        Ok(())
    }
}
