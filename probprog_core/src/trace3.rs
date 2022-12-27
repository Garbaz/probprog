use crate::distributions::{bernoulli::{BernoulliParams, Bernoulli}, uniform::{UniformParams, Uniform01}};


enum PrimitiveSupportTypes {
    Bool(bool),
    F64(f64),
}

enum PrimitiveParamTypes {
    Bernoulli(BernoulliParams),
    Uniform01(UniformParams),
}

enum PrimitiveDistributionTypes {
    Bernoulli(Bernoulli),
    Uniform01(Uniform01),
}