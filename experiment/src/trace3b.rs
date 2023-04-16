use std::collections::VecDeque;

// use crate::distribution::{ParametrizedValue, Sample};

// #[derive(Debug, Clone)]
// pub struct TraceEntry {
//     pub sample: ParametrizedSample,
//     pub touched: bool,
// }

// impl TraceEntry {
//     pub fn new(sample : ParametrizedSample) -> Self {
//         Self {
//             sample,
//             touched: true,
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct Dummy;

type ParametrizedSample = Dummy;

#[derive(Debug, Clone)]
pub enum Trace {
    Primitive { sample: ParametrizedSample },
    Function { name: String, subtraces: Traces },
    Loop { iteration: usize, subtraces: Traces },
}

impl Trace {
    fn subtraces(&mut self) -> Option<&mut Traces> {
        match self {
            Trace::Function { name, subtraces } => Some(subtraces),
            Trace::Loop {
                iteration,
                subtraces,
            } => Some(subtraces),
            _ => None,
        }
    }

    fn function_subtraces<N : PartialEq<String>>(self, name: N) -> Traces {
        match self {
            Trace::Function {
                name: name_,
                subtraces,
            } if name == name_ => subtraces,
            _ => Traces::new(),
        }
    }

    fn loop_subtraces(self, iteration: usize) -> Traces {
        match self {
            Trace::Loop {
                iteration: iteration_,
                subtraces,
            } if iteration_ == iteration => subtraces,
            _ => Traces::new(),
        }
    }

    fn primitive_sample(self) -> Option<ParametrizedSample> {
        match self {
            Trace::Primitive { sample } => Some(sample),
            _ => None
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct Traces(VecDeque<Trace>);
type Traces = VecDeque<Trace>;

trait PushBackAndMut<T> {
    fn push_back_and_mut(&mut self, value: T) -> &mut T;
}

impl<T> PushBackAndMut<T> for VecDeque<T> {
    fn push_back_and_mut(&mut self, value: T) -> &mut T {
        self.push_back(value);
        self.back_mut().unwrap()
    }
}

mod blorbo {
    use super::*;

    fn f(__trace: Trace) -> Trace {
        let __function_name = "f";

        let mut __new_trace = Trace::Function {
            name: __function_name.to_string(),
            subtraces: Traces::new(),
        };


        let return_value = (|| {
            let mut __traces = __trace.function_subtraces(__function_name);
            let __new_traces = __new_trace.subtraces().unwrap();
            // let __new_traces = match __

            let x = {
                let s = __traces.pop_front();
                
            };

            // let mut c = 0;

            // loop {
            //     let b = {
            //         let sample = match __new_trace.pop_back() {
            //             // Ensure here that it's the right kind of sample &
            //             // update it
            //             Some(sample) if true => sample,
            //             _ => Sample {
            //                 value: DummyValue,
            //                 log_probability: 0.,
            //             },
            //         };

            //         __new_trace.push_variable(sample);

            //         // sample.value
            //         true
            //     };

            //     if b {
            //         return c;
            //     } else {
            //         c += 1;
            //     }
            // }
        })();
        __new_trace
    }
}

// impl Traces {
//     pub fn new() -> Self {
//         Self(VecDeque::new())
//     }

//     pub fn push_function<N: ToString>(
//         &mut self,
//         name: N,
//         subtraces: Traces,
//     ) -> &mut Traces {
//         let e = self.0.push_back_and_mut(Trace::Function {
//             name: name.to_string(),
//             subtraces,
//         });
//         if let Trace::Function {
//             subtraces: subtrace,
//             ..
//         } = e
//         {
//             subtrace
//         } else {
//             unreachable!()
//         }
//     }

//     pub fn push_loop(
//         &mut self,
//         iteration: usize,
//         subtraces: Traces,
//     ) -> &mut Traces {
//         let e = self.0.push_back_and_mut(Trace::Loop {
//             iteration,
//             subtraces,
//         });
//         if let Trace::Loop {
//             subtraces: subtrace,
//             ..
//         } = e
//         {
//             subtrace
//         } else {
//             unreachable!()
//         }
//     }

//     pub fn push_variable(&mut self, sample: ParametrizedSample) {
//         self.0.push_back(Trace::Variable { sample })
//     }

//     pub fn pop_function<N: PartialEq<String>>(
//         &mut self,
//         name: N,
//     ) -> Option<Traces> {
//         match self.0.pop_front() {
//             Some(Trace::Function {
//                 name: name_,
//                 subtraces,
//             }) if name == name_ => Some(subtraces),
//             _ => None,
//         }
//     }

//     pub fn pop_loop(&mut self, iteration: usize) -> Option<Traces> {
//         match self.0.pop_front() {
//             Some(Trace::Loop {
//                 iteration: iteration_,
//                 subtraces,
//             }) if iteration == iteration_ => Some(subtraces),
//             _ => None,
//         }
//     }

//     pub fn pop_variable(&mut self) -> Option<ParametrizedSample> {
//         match self.0.pop_front() {
//             Some(Trace::Variable { sample }) => Some(sample),
//             _ => None,
//         }
//     }
// }