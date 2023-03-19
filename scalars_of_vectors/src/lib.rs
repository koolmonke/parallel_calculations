use rayon::prelude::*;

/// ```
/// use scalars_of_vectors::scalar;
/// let a = vec![1.0, 2.0, 3.0];
/// let b = vec![3.0, 4.0, 5.0];
/// assert_eq!(scalar(&a,&b), 26.0);
/// ```
pub fn scalar(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    v1.par_iter().zip(v2.par_iter()).map(|(a, b)| a * b).sum()
}
