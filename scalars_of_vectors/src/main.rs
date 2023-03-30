use bencher::rayon::{bench, bench_single_threaded};
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};
use rayon::prelude::*;
use scalars_of_vectors::scalar;

const VECTOR_SIZE: usize = 200_000_000;
const THREAD_COUNT: [usize; 3] = [4, 8, 16];
const REPEAT_COUNT: i32 = 10;

fn generate_vector(n: usize) -> Vec<i64> {
    let between = Uniform::from(0..10_000);
    let mut v = Vec::with_capacity(n);
    (0..n)
        .into_par_iter()
        .map(|_| between.sample(&mut thread_rng()))
        .collect_into_vec(&mut v);
    v
}

fn main() {
    let a = &generate_vector(VECTOR_SIZE);
    let b = &generate_vector(VECTOR_SIZE);

    let single_threaded_result = bench_single_threaded(
        move || {
            scalar(a, b);
        },
        REPEAT_COUNT,
    );

    println!("{}\n", single_threaded_result);
    for thread_count in THREAD_COUNT {
        let bench_result = bench(
            move || {
                scalar(a, b);
            },
            thread_count,
            REPEAT_COUNT,
            &single_threaded_result,
        );
        println!("{}\n", bench_result);
    }
}
