use crate::new_structure2::{Distribution, FnProb, Sample, TracedSample};

struct MH<T, F: FnProb<T>> {
    probfunc: F,
    traced_sample: TracedSample<T>,
}

impl<T: Clone, F: FnProb<T>> Iterator for MH<T, F> {
    type Item = Sample<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.traced_sample.trace.clean();

        // We can save some memory & performance if needed by not cloning here
        // but instead adding some junk to `Trace`. For now I'll leave it like
        // this.
        let mut trace = self.traced_sample.trace.clone();

        if let Some(wiggler) = trace.random_variable() {
            let proposal = wiggler.propose();

            let s = self.probfunc.resample(&mut trace);

            // The Metropoli-Hastings accept ratio
            let score = s.log_likelihood
                - self.traced_sample.sample.log_likelihood
                + proposal.backward_log_likelihood
                - proposal.forward_log_likelihood;

            if score > 0. || rand::random::<f64>().log2() < score {
                self.traced_sample.sample = s;
                self.traced_sample.trace = trace;
            }
        }

        Some(self.traced_sample.sample.clone())
    }
}

pub fn metropolis_hastings<T: Clone, F: FnProb<T>>(
    probfunc: F,
) -> impl Iterator<Item = Sample<T>> {
    MH {
        traced_sample: probfunc.sample(),
        probfunc,
    }
}

