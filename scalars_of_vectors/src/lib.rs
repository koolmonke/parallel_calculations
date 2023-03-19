use rayon::prelude::*;

/// ```
/// use scalars_of_vectors::scalar;
/// let a = vec![1, 2, 3];
/// let b = vec![3, 4, 5];
/// assert_eq!(scalar(&a,&b), 26);
/// ```
pub fn scalar(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    v1.par_iter().zip(v2.par_iter()).map(|(a, b)| a * b).sum()
}
