use bencher::generate_bench;
use cannon::generate_square_matrix;
use gauss::gaussian_elimination;
use scalars_of_vectors::generate_vector;

const MATRIX_SIZE: usize = 2000;

fn main() {
    let a = &generate_square_matrix::<f64>(MATRIX_SIZE, 1.0..10_000.0);
    let b = &generate_vector::<f64>(MATRIX_SIZE, &(1.0..10_000.0));

    generate_bench!(2, [4, 8, 16], gaussian_elimination, a, b);
}
