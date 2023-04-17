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
        // println!("{}", self.traced_sample);

        // We can save some memory & performance if needed by not cloning here
        // but instead adding some junk to `Trace`. For now I'll leave it like
        // this.
        let mut invalid_trace = self.traced_sample.trace.clone();
        let wiggler = invalid_trace.random_primitive();

        if let Some(wiggler) = wiggler {
            let proposal = wiggler.propose();

            let valid_traced_sample = self.distribution.resample(invalid_trace);

            let lp_choose_forward =
                -(self.traced_sample.trace.primitives().count() as f64).log2();
            let lp_choose_reverse =
                -(valid_traced_sample.trace.primitives().count() as f64).log2();

            let lp_propose_forward = proposal.forward_log_probability;
            let lp_propose_reverse = proposal.reverse_log_probability;

            let lp_kernel_forward = lp_choose_forward + lp_propose_forward;
            let lp_kernel_reverse = lp_choose_reverse + lp_propose_reverse;

            let lp_distribution_current =
                self.traced_sample.sample.log_probability;
            let lp_distribution_proposal =
                valid_traced_sample.sample.log_probability;

            let score = (lp_distribution_proposal + lp_kernel_reverse)
                - (lp_distribution_current + lp_kernel_forward);

            if score > 0. || rand::random::<f64>().log2() < score {
                self.traced_sample = valid_traced_sample;
            }
        }
        Some(self.traced_sample.sample.value.clone())
    }
}
