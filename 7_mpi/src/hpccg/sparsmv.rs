use rayon::prelude::*;
use super::SparseMatrix;

/// Sparse matrix-vector multiplication
///
/// # Arguments
/// * `matrix` - A representation of a sparse matrix.
/// * `vector` - The input vector to multiply the sparse matrix by.
pub fn sparsemv(matrix: &SparseMatrix, vector: &[f64]) -> Vec<f64> {
    matrix.row_start_inds.par_iter()
        .zip(matrix.nnz_in_row.par_iter())
        .map(
            |(&start_ind, &cur_nnz)| {
                    debug_assert!(start_ind + cur_nnz <= matrix.list_of_vals.len());
                    debug_assert!(start_ind + cur_nnz <= matrix.list_of_inds.len());
                    debug_assert!(matrix.list_of_inds[start_ind + cur_nnz - 1] <= vector.len());
                    let mut sum = 0.0;
                    for j in 0..cur_nnz {
                        sum += unsafe {
                            matrix.list_of_vals.get_unchecked(start_ind + j)
                                * vector.get_unchecked(*matrix.list_of_inds.get_unchecked(start_ind + j))
                        };
                    }
                    sum
                }
                // (start_ind..start_ind+cur_nnz).iter().map(
                //     |i| unsafe {
                //         matrix.list_of_vals.get_unchecked(i)
                //             * vector.get_unchecked(*matrix.list_of_inds.get_unchecked(i))
                //     }
                // ).sum()
                // matrix.list_of_vals[start_ind..(start_ind+cur_nnz)].iter()
                //     .zip(matrix.list_of_inds[start_ind..(start_ind+cur_nnz)].iter())
                //     .map(
                //         |(matrix_val, &vector_ind)|
                //             matrix_val * unsafe { vector.get_unchecked(vector_ind) }
                //     ).sum()
        ).collect()

    // let nrow = matrix.local_nrow;
    // let mut result = Vec::with_capacity(vector.len());
    // for i in 0..nrow {
    //     let mut sum = 0.0;
    //     debug_assert!(nrow <= matrix.row_start_inds.len());
    //     debug_assert!(nrow <= matrix.nnz_in_row.len());
    //     let start_ind = unsafe { *matrix.row_start_inds.get_unchecked(i) };
    //     let cur_nnz = unsafe { *matrix.nnz_in_row.get_unchecked(i) };
    //     debug_assert!(start_ind + cur_nnz <= matrix.list_of_vals.len());
    //     for j in 0..cur_nnz {
    //         sum += unsafe {
    //             matrix.list_of_vals.get_unchecked(start_ind + j)
    //                 * vector.get_unchecked(*matrix.list_of_inds.get_unchecked(start_ind + j))
    //         };
    //     }
    //     result.push(sum);
    // }
    // result
}