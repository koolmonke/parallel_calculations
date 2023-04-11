use common::matrix::Matrix;
use rayon::prelude::*;
use std::cmp::Ordering;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauss() {
        let matrix = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];
        let expected_result = vec![
            vec![1.0, 1.1428571428571428, 1.2857142857142858],
            vec![0.0, 1.0, 1.9999999999999998],
            vec![-0.0, -0.0, 1.0],
        ];
        let actual_result = gauss_elimination(&matrix);
        assert_eq!(expected_result, actual_result);
    }
}
