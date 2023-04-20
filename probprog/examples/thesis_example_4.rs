use probprog::{
    inference, normal, observe, prob, sample, uniform,
    visualization::simple_bar_graph,
};

/// What might have been the start position of a random walk,
/// given we know the end position and the number of steps?
#[prob]
fn example4(steps: u64, end_pos: f64) -> f64 {
    let start_pos = sample!(uniform(-10., 10.));
    let mut pos = start_pos;
    for _ in 0..steps {
        pos += sample!(normal(0., 0.5));
    }
    observe!(normal(pos, 1.), end_pos);
    start_pos
}

fn main() {
    let n = 100000;
    let burn_in = n / 2;

    println!(
        "{}",
        simple_bar_graph(
            80,
            20,
            inference(example4(3, 0.)).skip(burn_in).take(n)
        )
    );
}
