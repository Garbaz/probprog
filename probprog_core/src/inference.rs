use crate::trace::{Trace, TraceEntry};

#[derive(Debug)]
pub struct TracingData {
    pub path: String,
    pub trace: Trace,
    pub proposal: Option<(String, TraceEntry)>,
    // pub proposal_log_likelihood: f64,
}

impl TracingData {
    // pub fn new(proposal_name : String, proposal: DatabaseEntry) -> Self {
    //     InferenceConfig {
    //         tracedb: Database::new(),
    //         path: String::new(),
    //         proposal: Some((proposal_name, proposal)),
    //     }
    // }

    pub fn new() -> Self {
        TracingData {
            path: String::new(),
            trace: Trace::new(),
            proposal: None,
        }
    }
}
