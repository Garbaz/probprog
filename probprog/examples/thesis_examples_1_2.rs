use probprog::{bernoulli, inference, prob, sample};

/// Sampling from a primitve distribution and using recursion
#[prob]
fn example1(p: f64) -> u64 {
    let mut c = 0;
    while sample!(bernoulli(p)) {
        c += 1;
    }
    c
}

/// Sampling from another probabilistic program and using
/// conditionals & recursion
#[prob]
fn example2(n: u64) -> u64 {
    if sample!(example1(1. / (n as f64))) >= n {
        0
    } else {
        1 + sample!(example2(n))
    }
}

fn main() {
    let n = 1000;
    let burn_in = n / 2;

    println!(
        "{}",
        inference(example2(3))
            .skip(burn_in)
            .take(n)
            .map(|c| (c as f64) / (n as f64))
            .sum::<f64>()
    );
}
