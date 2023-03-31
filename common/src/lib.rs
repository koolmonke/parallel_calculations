use rand::{distributions::Uniform, prelude::Distribution, thread_rng};
use rayon::prelude::*;

pub fn generate_vector(n: usize) -> Vec<i64> {
    let between = Uniform::from(0..10_000);
    let mut v = Vec::with_capacity(n);
    (0..n)
        .into_par_iter()
        .map(|_| between.sample(&mut thread_rng()))
        .collect_into_vec(&mut v);
    v
}
