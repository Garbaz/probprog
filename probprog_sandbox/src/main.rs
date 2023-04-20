use probprog::{bernoulli, inference, o, prob, s, uniform};

#[prob]
fn probfunc1(p: f64) -> usize {
    let mut c = 0;
    loop {
        let x = s!(bernoulli(p));

        if x {
            c += 1;
        } else {
            return c;
        }
    }
}

#[prob]
fn probfunc2(obs: Vec<bool>) -> f64 {
    let p = s!(uniform(0., 1.));

    for o in &obs {
        o!(bernoulli(p), o);
    }

    p
}

fn main() {
    let n = 100000;
    let burn_in = n / 2;

    let obs = vec![true, false, true, true, false, true, false];

    let f = probfunc2(obs);

    let samples: Vec<_> = inference(f).skip(burn_in).take(n).collect();
    let avg: f64 = samples.iter().map(|x| *x / (n as f64)).sum();
    println!("{}", avg);
}
