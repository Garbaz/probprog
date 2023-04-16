use std::{collections::BTreeMap, fmt, ops::AddAssign};

use rand::{thread_rng, Rng};

use crate::distribution::{ParametrizedValue, Sample};

#[derive(Debug, Clone)]
pub struct TraceEntry {
    pub path: TracePath,
    pub sample: Sample<ParametrizedValue>,
    pub touched: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TraceDirectory {
    Function(String),
    Recursion,
    Loop(usize),
}

impl fmt::Display for TraceDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TraceDirectory::Function(n) => write!(f, "{}", n),
            TraceDirectory::Recursion => write!(f, "."),
            TraceDirectory::Loop(c) => write!(f, "@{}", c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TracePath(Vec<TraceDirectory>);

impl TracePath {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl AddAssign for TracePath {
    fn add_assign(&mut self, mut rhs: Self) {
        self.0.append(&mut rhs.0);
    }
}

impl fmt::Display for TracePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for d in &self.0 {
            write!(f, "{}/", d)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Trace {
    /// The trace entry in chronological order
    trace: Vec<TraceEntry>,
    /// A path index into the trace entry list
    index: BTreeMap<TracePath, usize>,
}

impl Trace {
    pub fn new() -> Self {
        Self {
            trace: Vec::new(),
            index: BTreeMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        path: TracePath,
        sample: Sample<ParametrizedValue>,
    ) {
        let entry = TraceEntry {
            path: path.clone(),
            sample,
            touched: true,
        };
        self.trace.push(entry);
        self.index.insert(path, self.trace.len() - 1);
    }

    pub fn attach(&mut self, path: TracePath, mut subtrace: Trace) {
        let l = self.trace.len();
        self.trace.append(&mut subtrace.trace);
        for (mut p, i) in subtrace.index {
            let mut path_p = path.clone();
            path_p += p;
            self.trace[i + l].path = path_p;
            self.index.insert(path_p, i + l);
        }
    }

    pub fn get(&self, path: &TracePath) -> Option<&TraceEntry> {
        let i = self.index.get(path)?;
        Some(&self.trace[*i])
    }

    pub fn get_mut(&mut self, path: &TracePath) -> Option<&mut TraceEntry> {
        let i = self.index.get(path)?;
        Some(&mut self.trace[*i])
    }

    pub fn iter(&self) -> impl Iterator<Item = &TraceEntry> {
        self.trace.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut TraceEntry> {
        self.trace.iter_mut()
    }

    pub fn random_variable(&mut self) -> Option<&mut TraceEntry> {
        if self.trace.is_empty() {
            None
        } else {
            let i = thread_rng().gen_range(0..self.trace.len());
            Some(&mut self.trace[i])
        }
    }

    pub fn clean(&mut self) {
        let mut new_indices = Vec::new();
        {
            let mut i = 0;
            self.trace.retain(|x| {
                if x.touched {
                    new_indices.push(Some(i));
                    i += 1;
                    true
                } else {
                    new_indices.push(None);
                    false
                }
            });
        }
        for e in &mut self.trace {
            e.touched = false;
        }
        self.index.retain(|_, i| {
            if let Some(ni) = new_indices[*i] {
                *i = ni;
                true
            } else {
                false
            }
        });
    }
}

impl IntoIterator for Trace {
    type Item = TraceEntry;

    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.trace.into_iter()
    }
}

impl<'a> IntoIterator for &'a Trace {
    type Item = &'a TraceEntry;

    type IntoIter = core::slice::Iter<'a, TraceEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.trace.iter()
    }
}

impl<'a> IntoIterator for &'a mut Trace {
    type Item = &'a mut TraceEntry;

    type IntoIter = core::slice::IterMut<'a, TraceEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.trace.iter_mut()
    }
}

// impl fmt::Display for Trace {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let ppath = TracePath::new();
//         let
//         for e in self {
//             if ppath == e.path {
//                 for _ in e.path {
//                     write!("")
//                 }
//             }
//         }
//         Ok(())
//     }
// }
