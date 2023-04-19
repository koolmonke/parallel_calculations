use std::{
    iter::Sum,
    ops::{Mul, Range},
};

use rand::{
    distributions::{uniform::SampleUniform, Uniform},
    prelude::Distribution,
    thread_rng,
};
use rayon::prelude::*;

pub fn generate_vector<T>(n: usize, range: &Range<T>) -> Vec<T>
where
    <T as SampleUniform>::Sampler: Sync,
    T: SampleUniform + Send + Copy,
{
    let between = Uniform::from(Range {
        start: range.start.into(),
        end: range.end.into(),
    });
    let mut v = Vec::with_capacity(n);
    (0..n)
        .into_par_iter()
        .map_init(|| thread_rng(), |rng, _| between.sample(rng))
        .collect_into_vec(&mut v);
    v
}

pub fn scalar<'a, T>(v1: &'a Vec<T>, v2: &'a Vec<T>) -> T
where
    &'a T: Mul<Output = T>,
    T: Send + Sync + Sum<T>,
{
    v1.par_iter().zip(v2.par_iter()).map(|(a, b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar() {
        let a = vec![1, 2, 3];
        let b = vec![3, 4, 5];
        assert_eq!(scalar(&a, &b), 26);
    }
}
