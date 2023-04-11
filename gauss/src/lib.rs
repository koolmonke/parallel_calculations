use common::matrix::Matrix;
use std::cmp::Ordering;
use rayon::prelude::*;

pub fn gauss_elimination(matrix: &Matrix<f64>) -> Matrix<f64> {
    let mut result = matrix.clone();
    let n = result.len();

    for i in 0..n {
        let max_row = (i..n)
            .max_by_key(|&row| {
                result[row][i]
                    .abs()
                    .partial_cmp(&result[i][i].abs())
                    .unwrap_or(Ordering::Equal)
            })
            .unwrap();
        if max_row != i {
            result.swap(i, max_row);
        }

        let pivot = result[i][i];

        if pivot == 0.0 {
            panic!("Singular matrix");
        }

        result[i] = result[i].par_iter().map(|&x| x / pivot).collect();

        for j in (i + 1)..n {
            let factor = result[j][i] / result[i][i];
            result[j] = result[j]
                .par_iter()
                .zip(result[i].par_iter())
                .map(|(&x, &y)| x - y * factor)
                .collect();
        }
    }

    result
}
