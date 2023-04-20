use probprog::{
    bernoulli, condition, inference, normal, observe, prob, sample,
    visualization::simple_bar_graph,
};

/// Modelling heights of e.g. people with a normal distribution
/// around some mean value.
/// However, a person's height can never be negative!
#[prob]
fn example5(mean_height: f64, deviation: f64) -> f64 {
    let height = sample!(normal(mean_height, deviation));
    condition!(height > 0.);
    height
}

/// Instead of the condition expression, we could also simply
/// observe the value of our expression from a `bernoulli(1.)`
/// distribution.
#[prob]
fn example5b(mean_height: f64, deviation: f64) -> f64 {
    let height = sample!(normal(mean_height, deviation));
    observe!(bernoulli(1.), height > 0.);
    height
}

/// We can even simply define our own `condition` as a
/// probabilistic program.
#[prob]
fn condition(c: bool) {
    observe!(bernoulli(1.), c);
}

#[prob]
fn example5c(mean_height: f64, deviation: f64) -> f64 {
    let height = sample!(normal(mean_height, deviation));
    sample!(condition(height > 0.));
    height
}

fn main() {
    let n = 10000;
    let burn_in = n / 2;

    println!(
        "example5:  All heights > 0: {}",
        inference(example5(1., 2.))
            .skip(burn_in)
            .take(n)
            .all(|h| h > 0.)
    );
    println!(
        "example5b: All heights > 0: {}",
        inference(example5b(1., 2.))
            .skip(burn_in)
            .take(n)
            .all(|h| h > 0.)
    );
    println!(
        "example5c: All heights > 0: {}",
        inference(example5c(1., 2.))
            .skip(burn_in)
            .take(n)
            .all(|h| h > 0.)
    );

    println!("");

    println!(
        "{}",
        simple_bar_graph(
            80,
            20,
            inference(example5(1., 2.)).skip(burn_in).take(n)
        )
    )
}
