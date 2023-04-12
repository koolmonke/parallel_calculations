pub mod matrix;

use std::ops::Range;

use matrix::Matrix;
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

pub fn generate_square_matrix<T>(n: usize, range: Range<T>) -> Matrix<T>
where
    <T as SampleUniform>::Sampler: Sync,
    T: SampleUniform + Send + Sync + Copy,
{
    let mut matrix = Vec::with_capacity(n);

    (0..n)
        .into_par_iter()
        .map(|_| generate_vector(n, &range))
        .collect_into_vec(&mut matrix);

    matrix
}
