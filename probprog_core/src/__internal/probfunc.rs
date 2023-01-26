use crate::__internal::trace::{TracingData, TracingPathRec};

pub struct ProbFunc<T, F>(pub(crate) F)
where
    F: Fn(&mut TracingPathRec, &mut TracingData) -> T;

impl<T, F> ProbFunc<T, F>
where
    F: Fn(&mut TracingPathRec, &mut TracingData) -> T,
{
    pub fn new(prob_func: F) -> Self {
        Self(prob_func)
    }
}

pub fn traced_sample<T, F>(
    prob_func: &mut ProbFunc<T, F>,
    tracing_path: &mut TracingPathRec,
    tracing_data: &mut TracingData,
) -> T
where
    F: Fn(&mut TracingPathRec, &mut TracingData) -> T,
{
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
