use std::{collections::VecDeque, iter};

use rand::{thread_rng, Rng};

use crate::distribution::{ParametrizedValue, Sample};

// use crate::distribution::{ParametrizedValue, Sample};

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
pub enum Trace {
    Primitive { sample: Sample<ParametrizedValue> },
    // Observe { sample: Sample<ParametrizedValue> },
    Function { name: String, subtraces: Traces },
    Loop { iteration: usize, subtraces: Traces },
    Empty,
}

impl Default for Trace {
    fn default() -> Self {
        Trace::Empty
    }
}

pub type Traces = VecDeque<Trace>;

impl Trace {
    pub fn subtraces(&mut self) -> Option<&mut Traces> {
        match self {
            Trace::Function { subtraces, .. } => Some(subtraces),
            Trace::Loop { subtraces, .. } => Some(subtraces),
            _ => None,
        }
    }

    pub fn function_subtraces<N: PartialEq<String>>(self, name: N) -> Traces {
        match self {
            Trace::Function {
                name: name_,
                subtraces,
            } if name == name_ => subtraces,
            _ => Traces::new(),
        }
    }

    pub fn loop_subtraces(self, iteration: usize) -> Traces {
        match self {
            Trace::Loop {
                iteration: iteration_,
                subtraces,
            } if iteration_ == iteration => subtraces,
            _ => Traces::new(),
        }
    }

    pub fn primitive_sample(self) -> Option<Sample<ParametrizedValue>> {
        match self {
            Trace::Primitive { sample } => Some(sample),
            _ => None,
        }
    }

    pub fn random_primitive(
        &mut self,
    ) -> Option<&mut Sample<ParametrizedValue>> {
        let mut primitives: Vec<_> = self.primitives_mut().collect();
        if primitives.is_empty() {
            None
        } else {
            let n = thread_rng().gen_range(0..primitives.len());
            Some(primitives.swap_remove(n))
        }
    }

    pub fn primitives_mut<'a>(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = &'a mut Sample<ParametrizedValue>> + 'a> {
        let fl = |subtraces: &'a mut Traces| {
            subtraces.iter_mut().flat_map(|t| t.primitives_mut())
        };

        match self {
            Trace::Primitive { sample } => Box::new(iter::once(sample)),
            Trace::Function { subtraces, .. } => Box::new(fl(subtraces)),
            Trace::Loop { subtraces, .. } => Box::new(fl(subtraces)),
            _ => Box::new(iter::empty()),
        }
    }

    pub fn primitives<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a Sample<ParametrizedValue>> + 'a> {
        let fmp = |subtraces: &'a Traces| {
            subtraces.iter().flat_map(|t| t.primitives())
        };

        match self {
            Trace::Primitive { sample } => Box::new(iter::once(sample)),
            Trace::Function { subtraces, .. } => Box::new(fmp(subtraces)),
            Trace::Loop { subtraces, .. } => Box::new(fmp(subtraces)),
            _ => Box::new(iter::empty()),
        }
    }

    pub fn log_probability_given(&self, other: &Self) -> f64 {
        use Trace::*;

        let fl = |subtraces: &Traces, subtraces_: &Traces| {
            subtraces
                .iter()
                .zip(subtraces_.iter().chain(iter::repeat(&Empty)))
                .map(|(t, t_)| t.log_probability_given(t_))
                .sum()
        };

        match (self, other) {
            (Primitive { sample }, Primitive { sample: sample_ })
                if sample.value.value_eq(&sample_.value) =>
            {
                0.
            }
            (
                Function { name, subtraces },
                Function {
                    name: name_,
                    subtraces: subtraces_,
                },
            ) if name == name_ => fl(subtraces, subtraces_),
            (
                Loop {
                    iteration,
                    subtraces,
                },
                Loop {
                    iteration: iteration_,
                    subtraces: subtraces_,
                },
            ) if iteration == iteration_ => fl(subtraces, subtraces_),
            (t, _) => t.primitives().fold(0., |acc, s| acc + s.log_probability),
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct Traces(VecDeque<Trace>);

pub trait PushBackAndMut<T> {
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

//     fn f(__trace: Trace) -> Trace {
//         const __FUNCTION_NAME: &str = "f";

//         let mut __ntrace = Trace::Function {
//             name: __FUNCTION_NAME.to_string(),
//             subtraces: Traces::new(),
//         };

//         let return_value = (|| {
//             let mut __traces = __trace.function_subtraces(__FUNCTION_NAME);
//             let __ntraces = __ntrace.subtraces().unwrap();
//             // let __new_traces = match __

//             let x = {
//                 if let Some(Trace::Primitive { sample }) = __traces.pop_front()
//                 {
//                 }
//             };

//             // let mut c = 0;

//             // loop {
//             //     let b = {
//             //         let sample = match __new_trace.pop_back() {
//             //             // Ensure here that it's the right kind of sample &
//             //             // update it
//             //             Some(sample) if true => sample,
//             //             _ => Sample {
//             //                 value: DummyValue,
//             //                 log_probability: 0.,
//             //             },
//             //         };

//             //         __new_trace.push_variable(sample);

//             //         // sample.value
//             //         true
//             //     };

//             //     if b {
//             //         return c;
//             //     } else {
//             //         c += 1;
//             //     }
//             // }
//         })();
//         __ntrace
//     }
// }

// // impl Traces {
// //     pub fn new() -> Self {
// //         Self(VecDeque::new())
// //     }

// //     pub fn push_function<N: ToString>(
// //         &mut self,
// //         name: N,
// //         subtraces: Traces,
// //     ) -> &mut Traces {
// //         let e = self.0.push_back_and_mut(Trace::Function {
// //             name: name.to_string(),
// //             subtraces,
// //         });
// //         if let Trace::Function {
// //             subtraces: subtrace,
// //             ..
// //         } = e
// //         {
// //             subtrace
// //         } else {
// //             unreachable!()
// //         }
// //     }

// //     pub fn push_loop(
// //         &mut self,
// //         iteration: usize,
// //         subtraces: Traces,
// //     ) -> &mut Traces {
// //         let e = self.0.push_back_and_mut(Trace::Loop {
// //             iteration,
// //             subtraces,
// //         });
// //         if let Trace::Loop {
// //             subtraces: subtrace,
// //             ..
// //         } = e
// //         {
// //             subtrace
// //         } else {
// //             unreachable!()
// //         }
// //     }

// //     pub fn push_variable(&mut self, sample: Sample<ParametrizedValue>) {
// //         self.0.push_back(Trace::Variable { sample })
// //     }

// //     pub fn pop_function<N: PartialEq<String>>(
// //         &mut self,
// //         name: N,
// //     ) -> Option<Traces> {
// //         match self.0.pop_front() {
// //             Some(Trace::Function {
// //                 name: name_,
// //                 subtraces,
// //             }) if name == name_ => Some(subtraces),
// //             _ => None,
// //         }
// //     }

// //     pub fn pop_loop(&mut self, iteration: usize) -> Option<Traces> {
// //         match self.0.pop_front() {
// //             Some(Trace::Loop {
// //                 iteration: iteration_,
// //                 subtraces,
// //             }) if iteration == iteration_ => Some(subtraces),
// //             _ => None,
// //         }
// //     }

// //     pub fn pop_variable(&mut self) -> Option<Sample<ParametrizedValue>> {
// //         match self.0.pop_front() {
// //             Some(Trace::Variable { sample }) => Some(sample),
// //             _ => None,
// //         }
// //     }
// // }
