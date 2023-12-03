use rayon::prelude::*;
use super::hpc_sparse_matrix::HpcSparseMatrix;

pub fn sparsemv(matrix: &HpcSparseMatrix, x: &Vec<f64>) -> Vec<f64> {
    // matrix.data.iter().map(|row| {
    matrix.data.par_iter().map(|row| {
        row.iter().map(|(val, ind)| val * x[*ind]).sum()
    }).collect()
}

#[test]
fn test_sparsemv() {
    let (matrix, _, _, _) = HpcSparseMatrix::generate_matrix(2, 2, 2);
    let vx = vec![20.0; 8];
    let vy = sparsemv(&matrix, &vx);
    assert_eq!(vy, vec![400.0; 8]);

    let (matrix, _, _, _) = HpcSparseMatrix::generate_matrix(3, 3, 3);
    let vx = vec![20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 10.0, 1.0, 10.0, 16.0, 10.0, 16.0, 20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 20.0, 16.0, 20.0];
    let expected_vy = vec![461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 21.0, -385.0, 21.0, 287.0, 21.0, 287.0, 461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 461.0, 287.0, 461.0];
    let vy = sparsemv(&matrix, &vx);
    assert_eq!(vy, expected_vy);
}
