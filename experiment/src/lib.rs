// pub mod trace;

#[derive(Debug, Clone)]
pub struct Sample<T> {
    pub value: T,
    pub log_probability: f64,
}

#[derive(Debug, Clone)]
pub enum ParametrizedValue {
    Bernoulli { value: bool, p: f64 },
    Uniform { value: f64, from: f64, to: f64 },
    Normal { value: f64, mean: f64, std_dev: f64 },
}
