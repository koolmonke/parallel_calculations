use bencher::generate_bench;
use scalar::{generate_vector, scalar};

const VECTOR_SIZE: usize = 200_000_000;

fn main() {
    let a = &generate_vector::<i64>(VECTOR_SIZE, &(0..10_000));
    let b = &generate_vector::<i64>(VECTOR_SIZE, &(0..10_000));

    generate_bench!(10, [4, 8, 16], scalar, a, b);
}
