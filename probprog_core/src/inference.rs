use crate::trace::{Database, DatabaseEntry, TraceEntry};

#[derive(Debug)]
pub struct InferenceConfig {
    pub tracedb: Database,
    pub path: String,
    pub proposal: Option<(String, DatabaseEntry)>,
    // pub proposal_log_likelihood: f64,
}

impl InferenceConfig {
    // pub fn new(proposal_name : String, proposal: DatabaseEntry) -> Self {
    //     InferenceConfig {
    //         tracedb: Database::new(),
    //         path: String::new(),
    //         proposal: Some((proposal_name, proposal)),
    //     }
    // }

    pub fn new() -> Self {
        InferenceConfig {
            tracedb: Database::new(),
            path: String::new(),
            proposal: None,
        }
    }
}
