use super::hpc_sparse_matrix::HpcSparseMatrix;

/// Sparse matrix-vector multiplication, i.e. calculating `y = Ax`
///
/// # Arguments
/// * `matrix` - A representation of a sparse matrix
/// * `x` - The input vector to multiply the sparse matrix by
/// * `y` - The output vector from the multiplication
pub fn sparsemv_direct(matrix: HpcSparseMatrix, x: &[f64], y: &mut [f64]) -> u32 {
    let nrow = matrix.local_nrow as usize;

    let mut start_index = 0;
    for i in 0..nrow {
        let mut sum = 0.0;
        // cur_vals is a vector slice with ptr_to_vals_in_row of length cur_nnz,
        // representing the values in the row
        // let cur_vals = matrix.ptr_to_vals_in_row[i];
        // let cur_inds = matrix.ptr_to_vals_in_row[i];
        let cur_nnz = matrix.nnz_in_row[i] as usize;

        for j in 0..cur_nnz {
            let val = *matrix.list_of_vals[start_index+j].borrow();
            let ind = *matrix.list_of_inds[start_index+j].borrow() as usize;
            sum += val * x[ind];
        }
        y[i] = sum;
        start_index += cur_nnz;
    }
    0
}


#[test]
fn test_sparse_matrix() {
    let (matrix, _, _, _) = HpcSparseMatrix::generate_matrix(2, 2, 2);
    let vx = [20.0; 8];
    let mut vy = [0.0; 8];
    sparsemv_direct(matrix, &vx, &mut vy);
    assert_eq!(vy, [400.0; 8]);

    let (matrix, _, _, _) = HpcSparseMatrix::generate_matrix(3, 3, 3);
    let vx = [20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 10.0, 1.0, 10.0, 16.0, 10.0, 16.0, 20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 20.0, 16.0, 20.0];
    let mut vy = [0.0; 27];
    let expected_vy = [461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 21.0, -385.0, 21.0, 287.0, 21.0, 287.0, 461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 461.0, 287.0, 461.0];
    sparsemv_direct(matrix, &vx, &mut vy);
    assert_eq!(vy, expected_vy);
}
