use crate::trace::TraceEntry;

pub trait Distribution {
    type SupportType;
    type ParamsType;

    fn sample(&self) -> Self::SupportType;
    // fn support(&self) -> Self::SupportType;
    fn params(&self) -> Self::ParamsType;
    fn trace(&self, current_value: Self::SupportType) -> TraceEntry;

    fn propose(&self, current_value : Self::SupportType) -> (f64, Self::SupportType);
    // fn propose_likelihood(&self, value : Self::SupportType) -> f64;
}