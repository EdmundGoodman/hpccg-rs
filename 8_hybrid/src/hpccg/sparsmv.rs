use rayon::prelude::*;
use super::SparseMatrix;

/// Sparse matrix-vector multiplication
///
/// # Arguments
/// * `matrix` - A representation of a sparse matrix.
/// * `vector` - The input vector to multiply the sparse matrix by.
pub fn sparsemv(matrix: &SparseMatrix, vector: &[f64]) -> Vec<f64> {
    matrix
        .row_start_inds
        .par_iter()
        .zip(matrix.nnz_in_row.par_iter())
        .map(|(&start_ind, &cur_nnz)| {
            debug_assert!(start_ind + cur_nnz <= matrix.list_of_vals.len());
            debug_assert!(start_ind + cur_nnz <= matrix.list_of_inds.len());
            debug_assert!(matrix.list_of_inds[start_ind + cur_nnz - 1] as usize <= vector.len());
            let mut sum = 0.0;
            for j in 0..cur_nnz {
                sum += unsafe {
                    matrix.list_of_vals.get_unchecked(start_ind + j)
                        * vector.get_unchecked(
                            *matrix.list_of_inds.get_unchecked(start_ind + j) as usize
                        )
                };
            }
            sum
        })
        .collect()
}
