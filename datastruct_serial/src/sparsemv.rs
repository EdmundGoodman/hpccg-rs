use super::hpc_sparse_matrix::HpcSparseMatrix;

pub fn sparsemv(matrix: &HpcSparseMatrix, x: &Vec<f64>) -> Vec<f64> {
    // let mut y = Vec::with_capacity(x.len());
    // // todo: debug mov command
    // for row in matrix.data.iter() {
    //     let mut sum = 0.0;
    //     for (val, ind) in row.iter() {
    //         sum += val * x[*ind];
    //     }
    //     y.push(sum);
    // }
    // y
    matrix.data.iter().map(|row| {
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
