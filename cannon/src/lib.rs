use rayon::prelude::*;

fn compute_row_of_sums_rayon(a_row: &Vec<f64>, b: &Vec<Vec<f64>>, n: usize) -> Vec<f64> {
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

pub fn multiply(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
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
