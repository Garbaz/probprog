struct Value<T> {
    value: T,
    loglikelihood: f64,
}

struct Trace;

trait Sample<T> {
    fn sample(&self, trace: &mut Trace) -> Value<T>;
}

trait Observe<T> {
    fn observe(&self, trace: &mut Trace, value: &T);
}

trait Propose<T> {
    fn propose(&self, prior: &T) -> Value<T>;
}
