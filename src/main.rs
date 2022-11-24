use probprog::{prob::{Oracle, ProbFn}, prob};
use rand::prelude::*;
use rand_distr::Normal;

struct MyOracle(Normal<f64>);

impl MyOracle {
    pub fn new() -> Self {
        MyOracle(Normal::new(0., 1.).unwrap())
    }
}

impl Oracle<(f64,)> for MyOracle {
    fn divine<R: Rng + ?Sized>(&self, rng: &mut R) -> (f64,) {
        (self.0.sample(rng),)
    }
}

#[prob(y ~ Normal(0, 1), z ~ Bernoulli(0.5))]
fn test(x : f64) -> f64 {
    if z {
        x + y
    }
    else {
        0.
    }
}

fn main() {
    let mut rng = thread_rng();

    let pfn = ProbFn {
        oracle: MyOracle::new(),
        function: |u: (f64,), o: (f64,)| u.0 + o.0,
    };

    for i in -10..11 {
        println!("{}", pfn.call((i as f64,), &mut rng));
    }
}
