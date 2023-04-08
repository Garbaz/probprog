pub trait Distribution: Clone {
    type ParamsType;
    type SupportType: Copy;

    fn sample(&self) -> Self::SupportType;
    fn params(&self) -> Self::ParamsType;

    fn log_likelihood(&self, value: Self::SupportType) -> f64;

    fn kernel_propose(&self, prior: Self::SupportType) -> Self::SupportType;
    fn kernel_log_likelihood(
        &self,
        prior: Self::SupportType,
        proposal: Self::SupportType,
    ) -> f64;
}
