use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use scalars_of_vectors::*;

fn make_pool(num_threads: usize) -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap()
}

fn generate_vector(n: usize) -> Vec<i32> {
    let between = Uniform::from(0..10_000);
    let mut rng = thread_rng();
    between.sample_iter(&mut rng).take(n).collect()
}

const VECTOR_SIZE: usize = 600_000_000;

fn scalar_tests(c: &mut Criterion) {
    let a = black_box(generate_vector(VECTOR_SIZE));
    let b = black_box(generate_vector(VECTOR_SIZE));
    let pool1 = make_pool(1);
    let pool4 = make_pool(4);
    let pool8 = make_pool(8);
    let pool16 = make_pool(16);

    c.bench_function("scalar 1 thread", |bencher| {
        bencher.iter(|| {
            let _t = pool1.install(|| scalar(&a, &b));
        })
    });
    c.bench_function("scalar 4 thread", |bencher| {
        bencher.iter(|| {
            let _t = pool4.install(|| scalar(&a, &b));
        })
    });
    c.bench_function("scalar 8 thread", |bencher| {
        bencher.iter(|| {
            let _t = pool8.install(|| scalar(&a, &b));
        })
    });
    c.bench_function("scalar 16 thread", |bencher| {
        bencher.iter(|| {
            let _t = pool16.install(|| scalar(&a, &b));
        })
    });
}

criterion_group!(benches, scalar_tests);
criterion_main!(benches);
