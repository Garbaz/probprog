use std::fmt::Debug;

use crate::distribution::{Distribution, FnProb, Sample, TracedSample};

struct MH<T, F: FnProb<T>> {
    fn_prob: F,
    traced_sample: TracedSample<T>,
}

impl<T: Clone + Debug, F: FnProb<T>> Iterator for MH<T, F> {
    type Item = Sample<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.traced_sample.trace.clean();

        // We can save some memory & performance if needed by not cloning here
        // but instead adding some junk to `Trace`. For now I'll leave it like
        // this.
        let mut trace = self.traced_sample.trace.clone();

        println!("{}", trace);
        if let Some(wiggler) = trace.random_variable() {
            let proposal = wiggler.propose();

            let proposal_sample = self.fn_prob.resample(&mut trace);

            // println!("{:?}", proposal_sample);
            // println!("{}", trace);

            let current_log_likelihood =
                self.traced_sample.sample.log_likelihood;
            let proposal_log_likelihood = proposal_sample.log_likelihood;
            let forward_log_likelihood = proposal.forward_log_likelihood;
            let backward_log_likelihood = proposal.backward_log_likelihood;

            // The Metropoli-Hastings accept ratio
            let score = (proposal_log_likelihood - current_log_likelihood)
                + (forward_log_likelihood - backward_log_likelihood);

            if score > 0. || rand::random::<f64>().log2() < score {
                self.traced_sample.sample = proposal_sample;
                self.traced_sample.trace = trace;
            }
        }

        Some(self.traced_sample.sample.clone())
    }
}

pub fn metropolis_hastings<T: Clone + Debug, F: FnProb<T>>(
    fn_prob: F,
) -> impl Iterator<Item = Sample<T>> {
    MH {
        traced_sample: fn_prob.sample(),
        fn_prob,
    }
}
