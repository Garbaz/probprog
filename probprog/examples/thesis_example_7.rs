use probprog::{bernoulli, normal, prob, sample};

/// Depending on whether we sample `true` or `false` from the
/// Bernoulli distribution, the second sample expression we
/// encounter could either be to again sample from a Bernoulli
/// distribution or to sample from a normal distribution.
#[prob]
fn example7() -> f64 {
    if sample!(bernoulli(0.1)) {
        if sample!(bernoulli(0.5)) {
            1.
        } else {
            -1.
        }
    } else {
        sample!(normal(0., 1.))
    }
}

fn main() {}
