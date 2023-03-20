use std::{iter::Sum, ops::Mul};

use rayon::prelude::*;

/// ```
/// use scalars_of_vectors::scalar;
/// let a = vec![1, 2, 3];
/// let b = vec![3, 4, 5];
/// assert_eq!(scalar(&a, &b), 26);
/// ```
pub fn scalar<'a, T: Send + Sync + Sum<T>>(v1: &'a Vec<T>, v2: &'a Vec<T>) -> T
where
    &'a T: Mul<Output = T>,
{
    v1.par_iter().zip(v2.par_iter()).map(|(a, b)| a * b).sum()
}
