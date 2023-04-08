use self::distribution::{Bernoulli, Uniform};

pub fn bernoulli(p: f64) -> Bernoulli {
    Bernoulli::new(p)
}

pub fn uniform(from: f64, to: f64) -> Uniform {
    Uniform::new(from, to)
}

pub mod distribution {
    use crate::{
        new_structure2::PrimitiveDistribution, trace::ParametrizedValue,
    };

    use rand::thread_rng;
    use rand_distr as rd;

    pub struct Bernoulli {
        pub dist: rd::Bernoulli,
        pub p: f64,
    }

    impl Bernoulli {
        pub fn new(p: f64) -> Self {
            Self {
                dist: rd::Bernoulli::new(p).unwrap(),
                p,
            }
        }
    }

    impl PrimitiveDistribution<bool> for Bernoulli {
        fn raw_sample(&self) -> bool {
            rd::Distribution::sample(&self.dist, &mut thread_rng())
        }
        fn log_likelihood(&self, value: &bool) -> f64 {
            match value {
                true => self.p.log2(),
                false => (1. - self.p).log2(),
            }
        }

        fn parametrized(&self, value: bool) -> ParametrizedValue {
            ParametrizedValue::Bernoulli { value, p: self.p }
        }
    }

    pub struct Uniform {
        pub dist: rd::Uniform<f64>,
        pub from: f64,
        pub to: f64,
    }

    impl Uniform {
        pub fn new(from: f64, to: f64) -> Self {
            Self {
                dist: rd::Uniform::new(from, to),
                from,
                to,
            }
        }
    }

    impl PrimitiveDistribution<f64> for Uniform {
        fn raw_sample(&self) -> f64 {
            rd::Distribution::sample(&self.dist, &mut thread_rng())
        }

        fn log_likelihood(&self, value: &f64) -> f64 {
            if &self.from < value && value <= &self.to {
                -((self.to - self.from).log2())
            } else {
                f64::NEG_INFINITY
            }
        }

        fn parametrized(&self, value: f64) -> ParametrizedValue {
            ParametrizedValue::Uniform {
                value,
                from: self.from,
                to: self.to,
            }
        }
    }
}
