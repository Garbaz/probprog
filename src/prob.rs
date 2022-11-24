use rand::Rng;

pub trait Oracle<T> {
    fn divine<R: Rng + ?Sized>(&self, rng: &mut R) -> T;
}

pub struct ProbFn<A, T, O: Oracle<T>, B> {
    pub oracle: O,
    pub function: fn(A, T) -> B,
}

impl<U, T, O, B> ProbFn<U, T, O, B>
where
    O: Oracle<T>,
{
    pub fn call<R: Rng + ?Sized>(&self, user: U, rng: &mut R) -> B {
        (self.function)(user, self.oracle.divine(rng))
    }
}

// pub trait Oracle<T> {
//     fn divine(&mut self) -> T;
// }

// impl<T, D: Distribution<T>> Oracle<T> for D {
//     fn divine(&mut self) -> T {
//         self.sample(&thread_rng())
//     }
// }

// impl<D : Distribution<T>, R: Rng, T> Oracle<T> for DistIter<D, R, T> {
//     fn divine(&mut self) -> T {
//         self.next().unwrap()
//     }
// }

// pub struct ProbArgs<U, T, O>
// where
//     O: Oracle<T>,
// {
//     pub user: U,
//     pub oracle: O,
//     phantom: core::marker::PhantomData<T>,
// }
