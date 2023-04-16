use crate::distribution::{ParametrizedValue, Sample};

// #[derive(Debug, Clone)]
// pub struct TraceEntry {
//     pub sample: Sample<ParametrizedValue>,
//     pub touched: bool,
// }

// impl TraceEntry {
//     pub fn new(sample : Sample<ParametrizedValue>) -> Self {
//         Self {
//             sample,
//             touched: true,
//         }
//     }
// }

struct DummySample;

enum TraceEntry {
    Variable {
        // sample: Sample<ParametrizedValue>,
        sample: DummySample,
        touched: bool,
    },
    Function {
        name: String,
        subtrace: Trace,
    },
    Loop {
        iteration: usize,
        subtrace: Trace,
    },
}

pub struct Trace(pub Vec<TraceEntry>);

impl Trace {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

mod blorbo {
    use super::*;

    fn f(trace: Trace) -> Trace {
        let mut __old_trace = trace;
        let mut __new_trace = Trace::new();

        // sample x
        let x = {
            if let Some(TraceEntry::Variable { sample, touched }) = __old_trace.0.pop() {
                // if sample is the right kind & we might have resampled if params not equal
                
                sample
            } else {

            }
        };

    }

    fn g(trace: Trace) -> Trace {}
}
