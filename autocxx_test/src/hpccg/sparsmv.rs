use super::SparseMatrix;

/// Sparse matrix-vector multiplication
///
/// # Arguments
/// * `matrix` - A representation of a sparse matrix.
/// * `vector` - The input vector to multiply the sparse matrix by.
pub fn sparsemv(matrix: &SparseMatrix, vector: &[f64]) -> Vec<f64> {
    let nrow = matrix.local_nrow;

    let mut result = Vec::with_capacity(vector.len());
    for i in 0..nrow {
        let mut sum = 0.0;
        let start_val_ind = matrix.ptr_to_vals_in_row[i];
        let start_ind_ind = matrix.ptr_to_inds_in_row[i];
        let cur_nnz = matrix.nnz_in_row[i];
        for j in 0..cur_nnz {
            sum += matrix.list_of_vals[start_val_ind + j]
                * vector[matrix.list_of_inds[start_ind_ind + j]];
        }
        result.push(sum);
    }

    result
}

#[test]
fn test_sparsemv() {
    let (matrix, _, _, _) = SparseMatrix::generate_matrix(2, 2, 2);
    let vx = vec![20.0; 8];
    let vy = sparsemv(&matrix, &vx);
    assert_eq!(vy, vec![400.0; 8]);

    let (matrix, _, _, _) = SparseMatrix::generate_matrix(3, 3, 3);
    let vx = vec![
        20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 10.0, 1.0, 10.0,
        16.0, 10.0, 16.0, 20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 20.0, 16.0, 20.0,
    ];
    let expected_vy = vec![
        461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 21.0,
        -385.0, 21.0, 287.0, 21.0, 287.0, 461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 461.0, 287.0,
        461.0,
    ];
    let vy = sparsemv(&matrix, &vx);
    assert_eq!(vy, expected_vy);
}
