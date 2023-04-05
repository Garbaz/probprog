//! An attempt to develop a better trace structure optimized for immutable
//! tracing.

use std::collections::BTreeMap;

use crate::new_structure2::Sample;

#[derive(Debug, Clone)]
pub struct TracedSample<T> {
    pub sample: Sample<T>,
    pub trace: Trace,
}

impl<T> TracedSample<T> {
    pub fn new(value: T, log_likelihood: f64) -> Self {
        Self {
            sample: Sample {
                value,
                log_likelihood,
            },
            trace: Trace::new(),
        }
    }

    pub fn plus(self, other: Self) -> Self {
        Self {
            sample: Sample {
                value: other.sample.value,
                log_likelihood: self.sample.log_likelihood
                    + other.sample.log_likelihood,
            },
            trace: self.trace.plus(other.trace),
        }
    }
}

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

    pub fn plus(mut self, other: Trace) -> Self {
        self.variables.extend(other.variables);
        for (d, t) in other.directories {
            self = self.attach(d, t);
        }
        self
    }

    pub fn attach(
        mut self,
        directory: TraceDirectory,
        subtrace: Trace,
    ) -> Self {
        let subtrace = if let Some(t) = self.directories.remove(&directory) {
            t.plus(subtrace)
        } else {
            subtrace
        };
        self.directories.insert(directory, subtrace);
        self
    }

    pub fn with(mut self, trace_entry: TraceEntry) -> Self {
        self.variables.push(trace_entry);
        self
    }

    pub fn with_by_path(
        mut self,
        mut trace_path: TracePath,
        trace_entry: TraceEntry,
    ) -> Self {
        match trace_path.pop() {
            None => self.with(trace_entry),
            Some(dir) => {
                let subtree = match self.directories.remove(&dir) {
                    Some(subtree) => subtree,
                    None => Self::new(),
                };
                self.directories
                    .insert(dir, subtree.with_by_path(trace_path, trace_entry));
                self
            }
        }
    }

    pub fn cleaned(mut self) -> Self {
        self.variables.retain(|e| e.touched);
        self.directories = self
            .directories
            .into_iter()
            .map(|(d, t)| (d, t.cleaned()))
            .collect();
        self
    }
}

impl From<TraceEntry> for Trace {
    fn from(trace_entry: TraceEntry) -> Self {
        Trace::new().with(trace_entry)
    }
}

impl From<Sample<ParametrizedValue>> for Trace {
    fn from(sample: Sample<ParametrizedValue>) -> Self {
        TraceEntry::new(sample.value, sample.log_likelihood).into()
    }
}

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// struct TraceLocation {
//     trace_path: TracePath,
//     variable_name: usize,
// }

// impl TraceLocation {
//     pub fn new() -> Self {
//         Self {
//             trace_path: TracePath::new(),
//             variable_name: 0,
//         }
//     }
// }

// struct Q<
//     'a,
//     S: Iterator<Item = (TraceLocation, &'a mut TraceEntry)>,
//     V: Iterator<Item = (TraceLocation, &'a mut TraceEntry)>,
// >(Option<S>, V);

// impl<
//         'a,
//         S: Iterator<Item = (TraceLocation, &'a mut TraceEntry)>,
//         V: Iterator<Item = (TraceLocation, &'a mut TraceEntry)>,
//     > Iterator for Q<'a, S, V>
// {
//     type Item = (TraceLocation, &'a mut TraceEntry);

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(s) = &mut self.0 {
//             s.next()
//         } else {
//             self.1.next()
//         }
//     }
// }

// struct IterMut<'a> {
//     current_path: TracePath,
//     variable_iter: iter::Enumerate<slice::IterMut<'a, TraceEntry>>,
//     directories_iter: btree_map::IterMut<'a, TraceDirectory, Trace>,
// }

// impl<'a> Iterator for IterMut<'a> {
//     type Item = (TraceLocation, &'a mut TraceEntry);

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.variable_iter.next() {
//             Some((name, entry)) => Some((
//                 TraceLocation {
//                     trace_path: self.current_path,
//                     variable_name: name,
//                 },
//                 entry,
//             )),
//             None => match self.directories_iter.next() {
//                 Some((subdir, subtrace)) => {
//                     self.current_path.push(*subdir);
//                     self.directories_iter = subtrace.directories.iter_mut();
//                     self.variable_iter =
//                         subtrace.variables.iter_mut().enumerate();
//                     self.next()
//                 }
//                 None => None,
//             },
//         }
//     }
// }

// struct TraceIterator {
//     trace : Trace,
// }

// impl Iterator for Trace {
//     type Item = (TraceLocation, TraceEntry);

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.variables.pop() {
//             Some(entry) => Some((TraceLocation { trace_path: todo!(), variable_name: todo!() })),
//             None => todo!(),
//         }
//     }
// }

// impl IntoIterator for Trace {
//     type Item = (TraceLocation, TraceEntry);

//     type IntoIter = iter::Map<
//         <Vec<TraceEntry> as IntoIterator>::IntoIter,
//         fn(TraceEntry) -> (TraceLocation, TraceEntry),
//     >;

//     fn into_iter(self) -> Self::IntoIter {
//         fn q(te: TraceEntry) -> (TraceLocation, TraceEntry) {
//             (TraceLocation::new(), te)
//         }
//         let variables = self.variables.into_iter().map(q);
//         // let directories = self.directories.into_iter().fold(iter::empty(), |acc, (d, t)| );
//         variables
//     }
// }

// pub fn iter_variables_mut(
//     &mut self,
// ) -> impl Iterator<Item = (TraceLocation, &mut Vec<TraceEntry>)>
// {
//     todo!()
// }

// fn iter_variables_mut_(&mut self) -> Box<dyn Iterator<Item = (TraceLocation, &mut Vec<TraceEntry>)>> {
//     todo!()
// }

// pub fn iter_mut(
//     &mut self,
// ) -> impl Iterator<Item = (TraceLocation, &mut TraceEntry)> {
//     self.iter_mut_(TracePath::new())
// }

// /// Assemble an iterator over all leafs of the trace tree. Instead of
// /// defining our own struct and implementing `Iterator` for it, I'm using
// /// existing combinators. The only problem is that we have to use `Box<dyn
// /// ...>` for the return type, which might impact performance. I'm not sure.
// /// It probably would be a better idea to do this with a custom struct afterall.
// fn iter_mut_(
//     &mut self,
//     current_path: TracePath,
// ) -> Box<dyn Iterator<Item = (TraceLocation, &mut TraceEntry)>> {
//     // Since we need to move current path into two closures, make a copy for
//     // the second one. Maybe there is a prettier way to do this?
//     let current_path_ = current_path.clone();

//     let subs = self.directories.iter_mut().flat_map(move |(d, t)| {
//         t.iter_mut_({
//             let mut p = current_path.clone();
//             p.push(d.clone());
//             p
//         })
//     });

//     let vars = self.variables.iter_mut().enumerate().map(move |(n, e)| {
//         (
//             TraceLocation {
//                 trace_path: current_path_.clone(),
//                 variable_name: n,
//             },
//             e,
//         )
//     });

//     Box::new(vars.chain(subs))
// }
