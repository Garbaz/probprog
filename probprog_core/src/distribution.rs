use crate::trace::TraceEntry;

pub trait Distribution {
    type SupportType;
    type ParamsType;

    fn sample(&self) -> Self::SupportType;
    // fn support(&self) -> Self::SupportType;
    fn params(&self) -> Self::ParamsType;
    fn trace(&self, value: Self::SupportType) -> TraceEntry;

    fn likelihood(&self, value: Self::SupportType) -> f64;

    fn propose(&self, _current_value: Self::SupportType) -> Self::SupportType;
    fn proposal_likelihood(&self, current_value : Self::SupportType, proposal : Self::SupportType) -> f64;
}