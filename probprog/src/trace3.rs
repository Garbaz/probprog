use std::collections::VecDeque;

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

#[derive(Debug, Clone)]
enum TraceEntry {
    Variable { sample: Sample<ParametrizedValue> },
    Function { name: String, subtrace: Trace },
    Loop { iteration: usize, subtrace: Trace },
}

#[derive(Debug, Clone)]
pub struct Trace(VecDeque<TraceEntry>);
// type Trace = VecDeque<TraceEntry>;

impl Trace {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push_function<N: ToString>(
        &mut self,
        name: N,
        subtrace: Trace,
    ) -> &mut Trace {
        let e = self.0.push_back_and_mut(TraceEntry::Function {
            name: name.to_string(),
            subtrace,
        });
        if let TraceEntry::Function { subtrace, .. } = e {
            subtrace
        } else {
            unreachable!()
        }
    }

    pub fn push_loop(
        &mut self,
        iteration: usize,
        subtrace: Trace,
    ) -> &mut Trace {
        let e = self.0.push_back_and_mut(TraceEntry::Loop {
            iteration,
            subtrace,
        });
        if let TraceEntry::Loop { subtrace, .. } = e {
            subtrace
        } else {
            unreachable!()
        }
    }

    pub fn push_variable(&mut self, sample: Sample<ParametrizedValue>) {
        self.0.push_back(TraceEntry::Variable { sample })
    }

    pub fn pop_function<N: PartialEq<String>>(
        &mut self,
        name: N,
    ) -> Option<Trace> {
        match self.0.pop_front() {
            Some(TraceEntry::Function {
                name: name_,
                subtrace,
            }) if name == name_ => Some(subtrace),
            _ => None,
        }
    }

    pub fn pop_loop(&mut self, iteration: usize) -> Option<Trace> {
        match self.0.pop_front() {
            Some(TraceEntry::Loop {
                iteration: iteration_,
                subtrace,
            }) if iteration == iteration_ => Some(subtrace),
            _ => None,
        }
    }

    pub fn pop_variable(&mut self) -> Option<Sample<ParametrizedValue>> {
        match self.0.pop_front() {
            Some(TraceEntry::Variable { sample }) => Some(sample),
            _ => None,
        }
    }
}

impl From<Sample<ParametrizedValue>> for Trace {
    fn from(sample: Sample<ParametrizedValue>) -> Self {
        let mut trace = Trace::new();
        trace.push_variable(sample);
        trace
    }
}

trait PushBackAndMut<T> {
    fn push_back_and_mut(&mut self, value: T) -> &mut T;
}

impl<T> PushBackAndMut<T> for VecDeque<T> {
    fn push_back_and_mut(&mut self, value: T) -> &mut T {
        self.push_back(value);
        self.back_mut().unwrap()
    }
}

// mod blorbo {
//     use super::*;

//     fn f(trace: Trace) -> (i64, Trace) {
//         let mut __old_trace = trace;
//         let mut __trace = Trace::new();
//         let return_value = (|| {
//             let __trace = {
//                 let name = "f";
//                 let subtrace =
//                     __trace.pop_function(name).unwrap_or(Trace::new());
//                 __trace.push_function(name, subtrace)
//             };

//             let mut c = 0;

//             loop {
//                 let b = {
//                     let sample = match __trace.pop_variable() {
//                         // Ensure here that it's the right kind of sample &
//                         // update it
//                         Some(sample) if true => sample,
//                         _ => Sample {
//                             value: DummyValue,
//                             log_probability: 0.,
//                         },
//                     };

//                     __trace.push_variable(sample);

//                     // sample.value
//                     true
//                 };

//                 if b {
//                     return c;
//                 } else {
//                     c += 1;
//                 }
//             }
//         })();
//         (return_value, __trace)
//     }
// }
