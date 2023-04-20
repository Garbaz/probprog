use std::{collections::VecDeque, fmt, iter};

use rand::{thread_rng, Rng};

use crate::{distribution::Sample, primitive::ParametrizedValue};

#[derive(Debug, Clone, Default)]
pub enum Trace {
    Primitive {
        sample: Sample<ParametrizedValue>,
    },
    Function {
        name: String,
        subtraces: Traces,
    },
    Loop {
        iteration: usize,
        subtraces: Traces,
    },
    #[default]
    Empty,
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
            // We go through all subtraces and other subtraces in order and
            // accumulate the probability of each subtrace given the respective
            // other subtrace. We extend the other subtraces by an infinite
            // number of empty traces in case there are more subtraces than
            // other subtraces.
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
                // If the a primitive with the same kind and value also appears
                // in the other trace, then it's deterministic.
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
            (t, _) => {
                // If the other trace doesn't match, then it's fully
                // non-deterministic.
                t.primitives().fold(0., |acc, s| acc + s.log_probability)
            }
        }
    }
}

impl fmt::Display for Trace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fl = |f: &mut fmt::Formatter<'_>, subtraces: &Traces| {
            let sts = subtraces.iter().map(|t| format!("{}", t));
            for st in sts {
                let mut stl = st.lines();
                stl.next().and_then(|st| writeln!(f, "+- {}", st).err());
                for l in stl {
                    writeln!(f, "|  {}", l)?;
                }
            }
            Ok(())
        };

        match self {
            Trace::Primitive { sample } => {
                writeln!(f, "{}", sample)?;
            }
            Trace::Function { name, subtraces } => {
                writeln!(f, "{}", name)?;
                fl(f, subtraces)?;
            }
            Trace::Loop {
                iteration,
                subtraces,
            } => {
                writeln!(f, "@{}", iteration)?;
                fl(f, subtraces)?;
            }
            Trace::Empty => {}
        }
        Ok(())
    }
}

pub trait PushBackAndMut<T> {
    fn push_back_and_mut(&mut self, value: T) -> &mut T;
}

impl<T> PushBackAndMut<T> for VecDeque<T> {
    fn push_back_and_mut(&mut self, value: T) -> &mut T {
        self.push_back(value);
        self.back_mut().unwrap()
    }
}
