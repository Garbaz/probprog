use probprog::{
    bernoulli, distribution::Distribution, prob, sample,
    visualization::simple_bar_graph,
};

/// Depending on how many times we drawn a `false` from
/// the Bernoulli distribution, a different number of sample
/// expressions is encountered during an execution.
#[prob]
fn example6(p: f64) -> usize {
    if sample!(bernoulli(p)) {
        0
    } else {
        1 + sample!(example6(p))
    }
}

fn main() {
    let traces = (0..100000)
        .map(|_| example6(0.5).sample().trace.primitives().count() as f64);

    println!("{}", simple_bar_graph(80, 20, traces));
}
