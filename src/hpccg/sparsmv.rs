use super::SparseMatrix;

use mpi::traits::*;
/// Sparse matrix-vector multiplication
///
/// # Arguments
/// * `matrix` - A representation of a sparse matrix.
/// * `vector` - The input vector to multiply the sparse matrix by.
pub fn sparsemv(matrix: &SparseMatrix, vector: &[f64], world: &impl Communicator) -> Vec<f64> {
    // matrix
    //     .row_start_inds
    //     .iter()
    //     .zip(matrix.nnz_in_row.iter())
    //     .map(|(&start_ind, &cur_nnz)| {
    //         debug_assert!(start_ind + cur_nnz <= matrix.list_of_vals.len());
    //         debug_assert!(start_ind + cur_nnz <= matrix.list_of_inds.len());
    //         debug_assert!(matrix.list_of_inds[start_ind + cur_nnz - 1] as usize <= vector.len());
    //         let mut sum = 0.0;
    //         for j in 0..cur_nnz {
    //             sum += unsafe {
    //                 matrix.list_of_vals.get_unchecked(start_ind + j)
    //                     * vector.get_unchecked(
    //                         *matrix.list_of_inds.get_unchecked(start_ind + j) as usize
    //                     )
    //             };
    //         }
    //         sum
    //     })
    //     .collect()
    let nrow = matrix.local_nrow;

    let mut result = Vec::with_capacity(vector.len());
    for i in 0..nrow {
        let mut sum = 0.0;
        debug_assert!(nrow <= matrix.row_start_inds.len());
        debug_assert!(nrow <= matrix.nnz_in_row.len());
        let start_ind = unsafe { *matrix.row_start_inds.get_unchecked(i) };
        let cur_nnz = unsafe { *matrix.nnz_in_row.get_unchecked(i) };
        debug_assert!(start_ind + cur_nnz <= matrix.list_of_vals.len());
        for j in 0..cur_nnz {
            // if world.rank() == 0 {
            //     println!(
            //         "cur_vals[{}]={:+.5e}",
            //         start_ind + j,
            //         matrix.list_of_vals[start_ind + j]
            //     );
            //     println!(
            //         "x[cur_inds[{}]={:+.5e}",
            //         start_ind + j,
            //         vector[matrix.list_of_inds[start_ind + j] as usize]
            //     );
            // }
            sum += unsafe {
                matrix.list_of_vals.get_unchecked(start_ind + j)
                    * vector
                        .get_unchecked(*matrix.list_of_inds.get_unchecked(start_ind + j) as usize)
            };
        }
        result.push(sum);
    }

    result
}
