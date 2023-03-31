use cannon::par_par_multiply_a_b_rayon;

fn main() {
    let a = vec![vec![1., 2., 3.], vec![1., 2., 3.], vec![1., 2., 3.]];
    let b = vec![vec![1., 2., 3.], vec![1., 2., 3.], vec![1., 2., 3.]];

    println!("{:?}", a);
    println!("{:?}", par_par_multiply_a_b_rayon(&a, &b))
}
