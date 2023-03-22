use std::{iter::Sum, ops::Mul};

use bencher::{bench_rayon, bench_rayon_single_threaded};
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};
use scalars_of_vectors::scalar;

fn generate_vector(n: usize) -> Vec<i32> {
    let between = Uniform::from(0..10_000);
    let mut rng = thread_rng();
    between.sample_iter(&mut rng).take(n).collect()
}

const VECTOR_SIZE: usize = 600_000_000;
const THREAD_COUNT: [usize; 3] = [4, 8, 16];

pub fn ten_scaler<'a, T: Send + Sync + Sum<T>>(v1: &'a Vec<T>, v2: &'a Vec<T>) -> ()
where
    &'a T: Mul<Output = T>,
{
    for _ in 0..10 {
        scalar(v1, v2);
    }
}

fn main() {
    let a = &generate_vector(VECTOR_SIZE);
    let b = &generate_vector(VECTOR_SIZE);

    let bench_result_single_threaded = bench_rayon_single_threaded(move || {
        ten_scaler(a, b);
    });

    println!("{}", bench_result_single_threaded);
    println!("");
    for i in THREAD_COUNT {
        let bench_result = bench_rayon(
            i,
            move || {
                ten_scaler(a, b);
            },
            bench_result_single_threaded.duration,
        );
        println!("{}", bench_result);
        println!("");
    }
}
