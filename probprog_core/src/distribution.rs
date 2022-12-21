use crate::trace::TraceEntry;

// pub trait DistributionCmp {
//     // TODO: Add macro to auto-implement these, or find yet a way for these to be
//     // default-implemented.
//     fn as_any(&self) -> &dyn Any;
//     fn type_eq(&self, other: &(dyn DistributionCmp)) -> bool;

//     // Maybe we should move this method to `Distribution` and make
//     // this trait a generic `DynamicTypeEq`.
//     fn params_eq(&self, other: &(dyn DistributionCmp)) -> bool;
// }

pub trait Distribution /* : DistributionCmp */ {
    type ParamsType;
    type SupportType;

    fn sample(&self) -> Self::SupportType;
    // fn support(&self) -> Self::SupportType;
    fn params(&self) -> Self::ParamsType;


    fn trace(
        &self,
        value: Self::SupportType,
        log_likelihood: f64,
    ) -> TraceEntry;

    fn log_likelihood(&self, value: Self::SupportType) -> f64;

    fn kernel_propose(&self, prior: Self::SupportType) -> Self::SupportType;
    fn kernel_log_likelihood(
        &self,
        prior: Self::SupportType,
        proposal: Self::SupportType,
    ) -> f64;
}
