use std::marker::PhantomData;

use crate::__internal::trace::{TracingData, TracingPathRec};

#[derive(Debug)]
pub struct ConditionError {
    pub expr: String,
}

impl ConditionError {
    pub fn new(condition_expr: &str) -> Self {
        Self {
            expr: condition_expr.to_string(),
        }
    }
}

pub trait ProbFn<T>:
    Fn(&mut TracingPathRec, &mut TracingData) -> Result<T, ConditionError>
{
}

impl<T, F> ProbFn<T> for F where
    F: Fn(&mut TracingPathRec, &mut TracingData) -> Result<T, ConditionError>
{
}

pub struct ProbFunc<T, F: ProbFn<T>>(pub(crate) F, PhantomData<T>);

impl<T, F: ProbFn<T>> ProbFunc<T, F> {
    pub fn new(prob_func: F) -> Self {
        Self(prob_func, PhantomData)
    }
}

pub fn traced_sample<T, F: ProbFn<T>>(
    prob_func: &mut ProbFunc<T, F>,
    tracing_path: &mut TracingPathRec,
    tracing_data: &mut TracingData,
) -> Result<T, ConditionError> {
    (prob_func.0)(tracing_path, tracing_data)
}

// pub fn __internal_untraced_sample<T, F>(prob_func: &mut ProbFunc<T, F>) -> T
// where
//     F: Fn(TracingPath, &mut TracingData) -> T,
// {
//     let tracing_path = TracingPath::new();
//     let mut tracing_data = TracingData::new();
//     (prob_func.0)(tracing_path, &mut tracing_data)
// }

// #[macro_export]
// macro_rules! sample {
//     ($expr:expr) => {
//         ::probprog::probfunc::_internal_sample(
//             $expr,
//             __tracing_path_placeholder,
//             __tracing_data_placeholder,
//         )
//     };
// }
