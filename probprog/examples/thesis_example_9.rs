use probprog::{
    bernoulli, distribution::Distribution, normal, prob, sample, uniform,
};

#[prob]
fn flip() -> bool {
    sample!(bernoulli(0.5))
}

/// A probabilistic program that samples from another
/// probabilistic program. A trace for this program
/// would look something like this:
///   example9
///   +- uniform(0,10) => 4.03 : 0.100
///   +- flip
///   |  +- bernoulli(0.5) => true : 0.500
///   +- normal(0,1) => -1.13 : 0.209
#[prob]
fn example9() -> f64 {
    let x = sample!(uniform(0., 10.));
    let y = if sample!(flip()) {
        sample!(normal(0., 1.))
    } else {
        sample!(uniform(-1., 1.))
    };
    x + y
}

fn main() {
    println!("{}", example9().sample().trace)
}
