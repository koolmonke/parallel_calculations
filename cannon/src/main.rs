use bencher::{rayon::*, THREAD_COUNT};
use cannon::multiply;
use common::generate_square_matrix;

const MATRIX_SIZE: usize = 1000;
const REPEAT_COUNT: i32 = 5;

fn main() {
    let a = &generate_square_matrix::<f64>(MATRIX_SIZE, 0.0..10_000.0);
    let b = &generate_square_matrix::<f64>(MATRIX_SIZE, 0.0..10_000.0);

    let single_threaded_result = bench_single_threaded(
        move || {
            multiply(a, b);
        },
        REPEAT_COUNT,
    );

    println!("{}\n", single_threaded_result);
    for thread_count in THREAD_COUNT {
        let bench_result = bench(
            move || {
                multiply(a, b);
            },
            thread_count,
            REPEAT_COUNT,
            &single_threaded_result,
        );
        println!("{}\n", bench_result);
    }
}
