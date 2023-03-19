use scalars_of_vectors::scalar;

fn main() {
    let a = vec![1.0, 2.0, 3.0];
    let b = vec![3.0, 4.0, 5.0];

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{}", scalar(&a, &b));
}
