use std::marker::PhantomData;

use crate::distribution::{Distribution, Sample, TracedSample};

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
    type Item = Sample<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // println!("{}", self.traced_sample);

        // We can save some memory & performance if needed by not cloning here
        // but instead adding some junk to `Trace`. For now I'll leave it like
        // this.
        let mut trace = self.traced_sample.trace.clone();

        if let Some(wiggler) = trace.random_variable() {
            let proposal = wiggler.propose();

            // println!("{} | {}", wiggler, proposal.sample);

            let proposal_result = self.distribution.resample(&mut trace);
            trace.clean();

            // println!("{:?}", proposal_sample);
            // println!("{}", trace);

            let current_log_probability =
                self.traced_sample.sample.log_probability;
            let proposal_log_probability = proposal_result.log_probability;
            let backward_log_probability = proposal.backward_log_probability;
            let forward_log_probability = proposal.forward_log_probability;

            // The Metropoli-Hastings accept ratio
            let score = (proposal_log_probability - current_log_probability)
                + (backward_log_probability - forward_log_probability);

            if score > 0. || rand::random::<f64>().log2() < score {
                self.traced_sample.sample = proposal_result;
                self.traced_sample.trace = trace;
            }
        }
        Some(self.traced_sample.sample.clone())
    }
}
