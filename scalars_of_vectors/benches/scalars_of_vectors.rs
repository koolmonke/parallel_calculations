use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use scalars_of_vectors::*;

fn make_pool(num_threads: usize) -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap()
}

fn generate_vector(n: usize) -> Vec<i32> {
    let mut rng = thread_rng();
    (0..n).map(|_| rng.gen()).collect()
}

static VECTOR_SIZE: usize = 600_000_000;

fn scalar_tests(c: &mut Criterion) {
    let a = black_box(generate_vector(VECTOR_SIZE));
    let b = black_box(generate_vector(VECTOR_SIZE));
    let pool1 = make_pool(1);
    let pool4 = make_pool(4);
    let pool8 = make_pool(8);
    let pool12 = make_pool(12);
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
    c.bench_function("scalar 12 thread", |bencher| {
        bencher.iter(|| {
            let _t = pool12.install(|| scalar(&a, &b));
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
