

struct CheckPureFuncWithArgs<F>(F)
where
    F: Fn(f64, bool, &mut String) -> f64;

struct CheckPureFuncWithoutArgs<F>(F)
where
    F: Fn() -> f64;

fn purefunc_checked(
    x: f64,
    y: bool,
    z: &mut String,
    // ) -> (f64, Box<dyn Fn(f64, bool, &mut String) -> f64>) {
    // ) -> f64 {
) -> (f64, impl Fn() -> f64) {
    // let f = CheckPureFuncWithArgs(|x: f64, y: bool, z: &mut String| {
    let g = CheckPureFuncWithoutArgs(move || {
        // println!("{}", z);
        // z.push_str("!!");
        if y {
            x
        } else {
            0.
        }
    });

    // ((f.0)(x, y, z), Box::new(f.0))
    // (g.0)()
    ((g.0)(), g.0)
}

fn f(x: i64, y: i64, op: &String) -> i64 {
    if op == "add" {
        x + y
    } else {
        x - y
    }
}

fn main() {
    let mut s = "Hello".to_string();
    let _ = purefunc_checked(1., true, &mut s);
    s.push_str(" World");
    let _ = purefunc_checked(1., true, &mut s);

    println!("{}", s);

    if (&s) == "Hello World" {
        println!("equal!");
    } else {
        println!("not equal!");
    }

    let mut op = "add".to_string();
    let a = f(17, 29, &op);
    op.push_str(", or rather, we shall not");
    let b = f(17,29, &op);
    // Even though x, y and op are \"the same\", the result is not,
    // since the value referenced by op has changed.
    assert_ne!(a, b);
}
