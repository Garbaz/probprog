use rand::thread_rng;
use rand_distr as rd;

trait Distribution {
    type SupportType;
    fn sample(&self) -> Self::SupportType;
    fn support(&self) -> Self::SupportType;
}

pub struct Bernoulli {
    dist: rd::Bernoulli,
}

impl Bernoulli {
    pub fn new(p: f64) -> Result<Self, rd::BernoulliError> {
        Ok(Bernoulli {
            dist: rd::Bernoulli::new(p)?,
        })
    }
}

impl Distribution for Bernoulli {
    type SupportType = bool;

    fn sample(&self) -> Self::SupportType {
        rd::Distribution::sample(&self.dist, &mut thread_rng())
    }

    fn support(&self) -> Self::SupportType {
        todo!()
    }
}

// pub struct Uniform(f64, f64);
// pub struct Normal(f64, f64);
// pub struct Categorical<T>(Vec<(T, f64)>);
