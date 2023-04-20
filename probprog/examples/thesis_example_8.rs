use probprog::{normal, prob, sample, uniform};

/// Depending on the value sampled from the uniform distribution
/// the parameters for the normal distribution differ.
#[prob]
fn example8(m: f64) -> f64 {
    let s = sample!(uniform(1., 10.));
    sample!(normal(m, s))
}

fn main() {}
