use std::{collections::BTreeMap, marker::PhantomData};
enum PrimitiveParams {}

// impl PrimitiveParams {
//     fn distribution(&self) -> impl Distribution {
//         todo!()
//     }
// }

enum PrimitiveValue {}

struct TracePath;
struct TraceEntry {
    params: PrimitiveParams,
    value: PrimitiveValue,
    log_likelihood: f64,
    touched: bool,
}
struct Trace(BTreeMap<TracePath, TraceEntry>);

trait Distribution {
    fn sample(&self) -> PrimitiveValue;
    fn params(&self) -> PrimitiveParams;

    fn log_likelihood(&self, value: PrimitiveValue) -> f64;

    fn kernel_propose(&self, prior: PrimitiveValue) -> PrimitiveValue;
    fn kernel_log_likelihood(
        &self,
        prior: PrimitiveValue,
        proposal: PrimitiveValue,
    ) -> f64;
}

struct DistributionS<D: Distribution>(D);

trait FnProb<T>: Fn(&mut Trace, &mut TracePath) -> Option<T> {}

impl<T, F> FnProb<T> for F where F: Fn(&mut Trace, &mut TracePath) -> Option<T> {}

struct FnProbS<T, F: FnProb<T>>(F, PhantomData<T>);

trait Sample<T> {
    fn traced_sample(&self, trace: &mut Trace, trace_path: &mut TracePath)
        -> T;
}

impl<T, D> Sample<T> for DistributionS<D> where D: Distribution {
    fn traced_sample(&self, trace: &mut Trace, trace_path: &mut TracePath)
        -> T {
        todo!()
    }
}
impl<T, F> Sample<T> for FnProbS<T, F> where F: FnProb<T> {
    fn traced_sample(&self, trace: &mut Trace, trace_path: &mut TracePath)
        -> T {
        todo!()
    }
}

trait Observe<T>: Sample<T> {
    fn traced_observe(
        &self,
        trace: &mut Trace,
        trace_path: &mut TracePath,
        observation: T,
    );
}

impl<T, D> Observe<T> for DistributionS<D> where D: Distribution {
    fn traced_observe(
        &self,
        trace: &mut Trace,
        trace_path: &mut TracePath,
        observation: T,
    ) {
        todo!()
    }
}