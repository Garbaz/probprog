use crate::trace::TraceEntry;

pub trait Distribution {
    type SupportType;
    type ParamsType : PartialEq;

    fn sample(&self) -> Self::SupportType;
    // fn support(&self) -> Self::SupportType;
    fn params(&self) -> Self::ParamsType;
    fn trace(&self, value: Self::SupportType, log_likelihood : f64) -> TraceEntry;

    fn log_likelihood(&self, value: Self::SupportType) -> f64;

    fn kernel_propose(&self, prior: Self::SupportType) -> Self::SupportType;
    fn kernel_log_likelihood(
        &self,
        prior: Self::SupportType,
        proposal: Self::SupportType,
    ) -> f64;
}
