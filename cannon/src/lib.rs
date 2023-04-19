use std::ops::Range;

use rayon::prelude::*;

use rand::distributions::uniform::SampleUniform;
use scalars_of_vectors::generate_vector;

pub fn generate_square_matrix<T>(n: usize, range: Range<T>) -> Matrix<T>
where
    <T as SampleUniform>::Sampler: Sync,
    T: SampleUniform + Send + Sync + Copy,
{
    let mut matrix = Vec::with_capacity(n);

    (0..n)
        .into_par_iter()
        .map(|_| generate_vector(n, &range))
        .collect_into_vec(&mut matrix);

    matrix
}

pub type Matrix<T> = Vec<Vec<T>>;

fn compute_row_of_sums_rayon(a_row: &Vec<f64>, b: &Matrix<f64>, n: usize) -> Vec<f64> {
    let mut unordered_columns = (0..n)
        .into_par_iter()
        .map(|j| (j, (0..n).map(|k| a_row[k] * b[k][j]).sum()))
        .collect::<Vec<(usize, f64)>>();

    unordered_columns
        .par_sort_by(|(left_size, _left), (right_size, _right)| left_size.cmp(right_size));

    unordered_columns
        .into_iter()
        .map(|(_, col_el)| col_el)
        .collect()
}

pub fn multiply(a: &Matrix<f64>, b: &Matrix<f64>) -> Matrix<f64> {
    let n = b.len();

    let mut unordered_rows = (0..n)
        .into_par_iter()
        .map(move |i| {
            let a_row = &a[i];

            (i, compute_row_of_sums_rayon(a_row, b, n))
        })
        .collect::<Vec<(usize, Vec<f64>)>>();

    unordered_rows
        .par_sort_by(|(left_size, _left), (right_size, _right)| left_size.cmp(right_size));

    unordered_rows.into_iter().map(|(_, row)| row).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_matrices() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let expected_result = vec![vec![19.0, 22.0], vec![43.0, 50.0]];
        let actual_result = multiply(&a, &b);
        assert_eq!(expected_result, actual_result);
    }
}
