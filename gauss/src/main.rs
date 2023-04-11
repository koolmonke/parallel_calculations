use bencher::{rayon::*, THREAD_COUNT};
use common::generate_square_matrix;
use gauss::gauss_elimination;

const MATRIX_SIZE: usize = 1000;
const REPEAT_COUNT: i32 = 2;

fn main() {
    let matrix = &generate_square_matrix::<f64>(MATRIX_SIZE, 1.0..10_000.0);

    let single_threaded_result = bench_single_threaded(
        move || {
            gauss_elimination(matrix);
        },
        REPEAT_COUNT,
    );

    println!("{}\n", single_threaded_result);
    for thread_count in THREAD_COUNT {
        let bench_result = bench(
            move || {
                gauss_elimination(matrix);
            },
            thread_count,
            REPEAT_COUNT,
            &single_threaded_result,
        );
        println!("{}\n", bench_result);
    }
}
