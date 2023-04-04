use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Sample<T> {
    pub value: T,
    pub log_likelihood: f64,
}

#[derive(Clone, Debug)]
pub enum TracedValue {
    Bernoulli(bool, (f64,)),
    Uniform(f64, (f64, f64)),
}

#[derive(Clone, Debug)]
pub struct TracedSample {
    pub value: TracedValue,
    pub log_likelihood: f64,
}

#[derive(Clone, Debug)]
pub struct TraceEntry {
    pub sample: TracedSample,
    pub touched: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TracePath;

impl TracePath {
    pub fn new() -> Self {
        TracePath
    }

    pub fn next_variable(&mut self) -> Self {
        todo!()
    }
}

pub struct Trace {
    pub trace: BTreeMap<TracePath, TraceEntry>,
    pub proposal: Option<(TracePath, TracedSample)>,
    pub total_log_likelihood: f64,
}

impl Trace {
    pub fn new() -> Self {
        Trace {
            trace: BTreeMap::new(),
            proposal: None,
            total_log_likelihood: 0.,
        }
    }

    pub fn insert(
        &mut self,
        trace_path: TracePath,
        traced_sample: TracedSample,
    ) {
        self.total_log_likelihood += traced_sample.log_likelihood;
        self.trace.insert(
            trace_path.clone(),
            TraceEntry {
                sample: traced_sample,
                touched: true,
            },
        );
    }

    pub fn get(&self, trace_path: &TracePath) -> Option<TracedSample> {
        if let Some((proposal_trace_path, proposal_traced_sample)) =
            &self.proposal
        {
            if proposal_trace_path == trace_path {
                Some(proposal_traced_sample.clone())
            } else {
                None
            }
        } else {
            self.trace
                .get(trace_path)
                .map(|TraceEntry { sample, .. }| sample.clone())
        }
    }

    pub fn propose(
        &mut self,
        trace_path: TracePath,
        traced_sample: TracedSample,
    ) {
        self.proposal = Some((trace_path, traced_sample));
    }

    pub fn cleanup(&mut self) {
        self.trace.retain(|_, e| e.touched);
        for e in self.trace.values_mut() {
            e.touched = false;
        }
    }
}

pub trait Distribution<T> {
    fn sample(&self) -> Sample<T>;
    fn traced_sample(
        &self,
        current_trace_path: &mut TracePath,
        trace: &mut Trace,
    ) -> T;
}

pub trait PrimitiveDistribution<T: Clone>: Distribution<T> {
    fn log_likelihood(&self, value: &T) -> f64;
    fn kernel_propose(&self, prior: &T) -> Sample<T>;
    fn pack_value(&self, value: T) -> TracedValue;
    fn traced_sample(
        &self,
        current_trace_path: &mut TracePath,
        trace: &mut Trace,
    ) -> T {
        let sample = self.sample();
        let traced_value = self.pack_value(sample.value.clone());
        let traced_sample = TracedSample {
            value: traced_value,
            log_likelihood: sample.log_likelihood,
        };

        trace.insert(current_trace_path.next_variable(), traced_sample);

        sample.value
    }
    fn traced_observe(
        &self,
        current_trace_path: &mut TracePath,
        trace: &mut Trace,
        value: T,
    ) {
        let log_likelihood = self.log_likelihood(&value);
        let traced_sample = TracedSample {
            value: self.pack_value(value.clone()),
            log_likelihood,
        };
        trace.insert(current_trace_path.next_variable(), traced_sample);
    }
}

pub trait ProbFn<T>: Fn(&mut TracePath, &mut Trace) -> T {}

impl<T, F> ProbFn<T> for F where F: Fn(&mut TracePath, &mut Trace) -> T {}

impl<T, F: ProbFn<T>> Distribution<T> for F {
    fn sample(&self) -> Sample<T> {
        let mut trace = Trace::new();
        let value = self(&mut TracePath::new(), &mut trace);
        Sample {
            value,
            log_likelihood: trace.total_log_likelihood,
        }
    }

    fn traced_sample(
        &self,
        current_trace_path: &mut TracePath,
        trace: &mut Trace,
    ) -> T {
        self(current_trace_path, trace)
    }
}

struct Bernoulli;

// impl PrimitiveDistribution<bool> for Bernoulli {
    
// }