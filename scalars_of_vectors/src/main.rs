use scalars_of_vectors::scalar;

fn main() {
    let a = vec![1, 2, 3];
    let b = vec![3, 4, 5];

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{}", scalar(&a, &b));
}
