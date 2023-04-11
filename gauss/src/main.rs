use gauss::gauss_elimination;

fn main() {
    let matrix = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];

    let result = gauss_elimination(&matrix);
    println!("{:?}", result);
}
