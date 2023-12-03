use super::hpc_sparse_matrix::{HpcSparseMatrix,IdiomaticHpcSparseMatrix};

/// Sparse matrix-vector multiplication, i.e. calculating `y = Ax`
///
/// # Arguments
/// * `matrix` - A representation of a sparse matrix
/// * `x` - The input vector to multiply the sparse matrix by
/// * `y` - The output vector from the multiplication
pub fn sparsemv_direct(matrix: &HpcSparseMatrix, x: &[f64], y: &mut [f64]) -> u32 {
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

pub fn sparsemv_direct_vec(matrix: &HpcSparseMatrix, x: &Vec<f64>) -> Vec<f64> {
    let nrow = matrix.local_nrow as usize;

    let mut y = vec![0.0; x.len()];
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
    y
}

pub fn sparsemv_idiomatic_vec(matrix: &IdiomaticHpcSparseMatrix, x: &Vec<f64>) -> Vec<f64> {
    let nrow = matrix.local_nrow as usize;

    let mut y = Vec::with_capacity(x.len());
    // todo: debug mov command
    for row in matrix.data.iter() {
        let mut sum = 0.0;
        for (val, ind) in row.iter() {
            sum += val * x[*ind];
        }
        y.push(sum);
    }
    y
}


#[test]
fn test_sparsemv_direct() {
    let (matrix, _, _, _) = HpcSparseMatrix::generate_matrix(2, 2, 2);
    let vx = [20.0; 8];
    let mut vy = [0.0; 8];
    sparsemv_direct(&matrix, &vx, &mut vy);
    assert_eq!(vy, [400.0; 8]);

    let (matrix, _, _, _) = HpcSparseMatrix::generate_matrix(3, 3, 3);
    let vx = [20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 10.0, 1.0, 10.0, 16.0, 10.0, 16.0, 20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 20.0, 16.0, 20.0];
    let mut vy = [0.0; 27];
    let expected_vy = [461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 21.0, -385.0, 21.0, 287.0, 21.0, 287.0, 461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 461.0, 287.0, 461.0];
    sparsemv_direct(&matrix, &vx, &mut vy);
    assert_eq!(vy, expected_vy);
}

#[test]
fn test_sparsemv_direct_vec() {
    let (matrix, _, _, _) = HpcSparseMatrix::generate_matrix(2, 2, 2);
    let vx = vec![20.0; 8];
    let vy = sparsemv_direct_vec(&matrix, &vx);
    assert_eq!(vy, vec![400.0; 8]);

    let (matrix, _, _, _) = HpcSparseMatrix::generate_matrix(3, 3, 3);
    let vx = vec![20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 10.0, 1.0, 10.0, 16.0, 10.0, 16.0, 20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 20.0, 16.0, 20.0];
    let expected_vy = vec![461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 21.0, -385.0, 21.0, 287.0, 21.0, 287.0, 461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 461.0, 287.0, 461.0];
    let vy = sparsemv_direct_vec(&matrix, &vx);
    assert_eq!(vy, expected_vy);
}
