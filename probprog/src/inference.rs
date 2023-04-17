use std::marker::PhantomData;

use crate::distribution::{Distribution, TracedSample};

pub struct MetropolisHastings<_Tag, T, D: Distribution<_Tag, T>> {
    distribution: D,
    traced_sample: TracedSample<T>,
    _phantom: PhantomData<_Tag>,
}

impl<_Tag, T, D: Distribution<_Tag, T>> MetropolisHastings<_Tag, T, D> {
    pub fn new(distribution: D) -> Self {
        Self {
            traced_sample: distribution.sample(),
            distribution,
            _phantom: PhantomData,
        }
    }
}

impl<_Tag, T: Clone, D: Distribution<_Tag, T>> Iterator
    for MetropolisHastings<_Tag, T, D>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // We copy the current trace and wiggle at some random primitive
        let mut invalid_trace = self.traced_sample.trace.clone();
        let wiggler = invalid_trace.random_primitive();
        if let Some(wiggler) = wiggler {
            let lp_old_wiggler_value = wiggler.log_probability;
            let proposal = wiggler.propose();
            let lp_new_wiggler_value = wiggler.log_probability;
            // We run the invalidated trace through the probprog again to get a
            // valid proposal trace
            let valid_traced_sample = self.distribution.resample(invalid_trace);

            // The probability of choosing the particular primitve
            // that chose to wiggle at, both in the forward and reverse
            // direction.
            let lp_choose_forward =
                -(self.traced_sample.trace.primitives().count() as f64).log2();
            let lp_choose_reverse =
                -(valid_traced_sample.trace.primitives().count() as f64).log2();

            // The probability of proposing the proposal value given the
            // current value, and the probability of proposing the current value
            // given the proposal value.
            let lp_propose_forward = proposal.forward_log_probability;
            let lp_propose_reverse = proposal.reverse_log_probability;

            // The probability of getting the particular proposal trace when
            // wiggling the chosen primitive in the current trace, and the
            // probability of getting the current trace when wiggling the chosen
            // primitive in the proposal trace.
            let lp_validify_forward = valid_traced_sample
                .trace
                .log_probability_given(&self.traced_sample.trace)
                - lp_new_wiggler_value;
            let lp_validify_backward = self
                .traced_sample
                .trace
                .log_probability_given(&valid_traced_sample.trace)
                - lp_old_wiggler_value;

            // The probability of proposing the proposal trace
            // given the current trace, and the probability of proposing the
            // current trace given the proposal trace.
            let lp_kernel_forward =
                lp_choose_forward + lp_propose_forward + lp_validify_forward;
            let lp_kernel_reverse =
                lp_choose_reverse + lp_propose_reverse + lp_validify_backward;

            // The distribution probability of the current trace, and the
            // distribution probability of the proposal trace.
            let lp_distribution_current =
                self.traced_sample.sample.log_probability;
            let lp_distribution_proposal =
                valid_traced_sample.sample.log_probability;

            // The complete MH acceptance ratio.
            let score = (lp_distribution_proposal + lp_kernel_reverse)
                - (lp_distribution_current + lp_kernel_forward);

            // Accept the proposed trace with probability `min(1, score)`.
            if score > 0. || rand::random::<f64>().log2() < score {
                self.traced_sample = valid_traced_sample;
            }
        }

        // Yield either the current value again, if the proposal failed, or the
        // value for the new trace.
        Some(self.traced_sample.sample.value.clone())
    }
}
