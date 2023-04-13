use rayon::prelude::*;

pub fn gaussian_elimination(matrix: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = matrix.len();

    let mut augmented_matrix: Vec<Vec<f64>> = matrix
        .iter()
        .zip(b.iter())
        .map(|(row, &b_i)| {
            let mut augmented_row = Vec::with_capacity(row.len() + 1);
            augmented_row.clone_from(row);
            augmented_row.push(b_i);
            augmented_row
        })
        .collect();

    for k in 0..n {
        let pivot = augmented_matrix[k][k];
        for i in k + 1..n {
            let factor = augmented_matrix[i][k] / pivot;

            for j in k..=n {
                augmented_matrix[i][j] -= factor * augmented_matrix[k][j];
            }
        }
    }

    let mut x: Vec<f64> = vec![0.0; n];
    for i in (0..n).rev() {
        let sum: f64 = augmented_matrix[i][((i + 1)..n)]
            .par_iter()
            .zip(x[(i + 1)..n].par_iter())
            .map(|(j, x_j)| j * x_j)
            .sum();
        x[i] = (augmented_matrix[i][n] - sum) / augmented_matrix[i][i];
    }

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaussian_elimination() {
        let a = vec![
            vec![2.0, 1.0, -1.0],
            vec![-3.0, -1.0, 2.0],
            vec![-2.0, 1.0, 2.0],
        ];
        let b = vec![8.0, -11.0, -3.0];
        let expected_x = vec![2.0, 3.0, -1.0];
        let x = gaussian_elimination(&a, &b);
        assert_eq!(x, expected_x);
    }
}
