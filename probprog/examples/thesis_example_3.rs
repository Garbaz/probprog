use probprog::{bernoulli, inference, observe, prob, sample, uniform};

/// What parameter `p` for a bernoulli distribution explains our
/// observed results best?
#[prob]
fn example3(obs: Vec<bool>) -> f64 {
    let p = sample!(uniform(0., 1.));
    for o in &obs {
        observe!(bernoulli(p), o);
    }
    p
}

fn main() {
    let n = 100000;
    let burn_in = n / 2;

    let obs = vec![
        true, false, true, false, false, false, false, false, true, false,
    ];

    println!(
        "p≈{}",
        inference(example3(obs))
            .skip(burn_in)
            .take(n)
            .map(|c| (c as f64) / (n as f64))
            .sum::<f64>()
    );
}
