use bencher::generate_bench;
use cannon::multiply;
use common::generate_square_matrix;

const MATRIX_SIZE: usize = 800;

fn main() {
    let a = &generate_square_matrix::<f64>(MATRIX_SIZE, 0.0..10_000.0);
    let b = &generate_square_matrix::<f64>(MATRIX_SIZE, 0.0..10_000.0);

    generate_bench!(5, [4, 16], multiply, a, b);
}
