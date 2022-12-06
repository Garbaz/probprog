use std::marker::PhantomData;

use probprog::{
    prob,
    prob::{Oracle, ProbFn},
};
use rand::prelude::*;
use rand_distr::Normal;

/* struct MyOracle(Normal<f64>);

impl MyOracle {
    pub fn new() -> Self {
        MyOracle(Normal::new(0., 1.).unwrap())
    }
}

impl Oracle<(f64,)> for MyOracle {
    fn divine<R: Rng + ?Sized>(&self, rng: &mut R) -> (f64,) {
        (self.0.sample(rng),)
    }
} */

trait Distribution<T> {
    fn sample(&self) -> T;
    fn trace(&self, result: &T) -> String;
}

struct TwoPoint<T>(T, T)
where
    T: Clone;

impl<T> Distribution<T> for TwoPoint<T>
where
    T: Clone + std::fmt::Display,
{
    fn sample(&self) -> T {
        let result = if random() {
            self.0.clone()
        } else {
            self.1.clone()
        };
        println!("{}", self.trace(&result));
        result
    }

    fn trace(&self, result: &T) -> String {
        format!("TwoPoint({}, {}) -> {}", self.0, self.1, result)
    }
}

#[prob]
fn test(x: f64) -> f64 {
    let y = TwoPoint(true, false).sample();
    if y {
        x
    } else {
        0.
    }
}

// fn probfunc(t : ProbfuncTrace, x : f64) {
//     let
// }

struct Fun<A, B, F: FnMut(A) -> B> {
    pub f: F,
    phantom: PhantomData<(A, B)>,
}

impl<A, B, F: FnMut(A) -> B> Fun<A, B, F> {
    fn new(f: F) -> Self {
        Self {
            f,
            phantom: PhantomData,
        }
    }
}

fn main() {
    // let mut rng = thread_rng();

    // let pfn = ProbFn {
    //     oracle: MyOracle::new(),
    //     function: |u: (f64,), o: (f64,)| u.0 + o.0,
    // };

    // for i in -10..11 {
    //     println!("{}", pfn.call((i as f64,), &mut rng));
    // }

    for _ in 0..10 {
        println!("{}", test(17.29));
    }
    let mut i = 0;
    let mut f = Fun::new(|x| {
        i += x;
        i
    });

    for _ in 0..10 {
        println!("{}", (f.f)(17));
    }
}
