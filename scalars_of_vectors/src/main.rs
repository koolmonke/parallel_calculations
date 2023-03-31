use bencher::{
    rayon::{bench, bench_single_threaded},
    THREAD_COUNT,
};
use common::generate_vector;
use scalars_of_vectors::scalar;

const VECTOR_SIZE: usize = 200_000_000;
const REPEAT_COUNT: i32 = 10;

fn main() {
    let a = &generate_vector::<i64>(VECTOR_SIZE, &(0..10_000));
    let b = &generate_vector::<i64>(VECTOR_SIZE, &(0..10_000));

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
