use super::SparseMatrix;

use mpi::traits::*;
use mpi::environment::Universe;
use mpi::collective::SystemOperation;
use std::collections::HashMap;

const MAX_EXTERNAL: usize = 100000;
const MAX_NUM_MESSAGES: usize = 500;
const MAX_NUM_NEIGHBORS: usize = MAX_NUM_MESSAGES;

const DEBUG: bool = false;
const DEBUG_DETAILS: bool = false;


pub fn make_local_matrix(matrix: &mut SparseMatrix, universe: &Universe) {}

fn _make_local_matrix(matrix: &mut SparseMatrix, universe: &Universe) {
    let mut externals: HashMap<usize, usize> = HashMap::new();
    let num_external: usize = 0;

    let world = universe.world();
    let size = world.size() as usize;
    let rank = world.rank() as usize;

    // We need to convert the index values for the rows on this processor
    // to a local index space. We need to:
    // - Determine if each index reaches to a local value or external value
    // - If local, subtract start_row from index value to get local index
    // - If external, find out if it is already accounted for.
    //   - If so, then do nothing,
    //   - otherwise
    //     - add it to the list of external indices,
    //     - find out which processor owns the value.
    //     - Set up communication for sparse MV operation.

    ///////////////////////////////////////////
    // Scan the indices and transform to local
    ///////////////////////////////////////////

    matrix.external_index = vec![];  //Vec::with_capacity(MAX_EXTERNAL);
    matrix.external_local_index = vec![];  //Vec::with_capacity(MAX_EXTERNAL);

    for i in 0..matrix.local_nrow {
        let row_start_ind = matrix.row_start_inds[i];
        for j in 0..matrix.nnz_in_row[i] {
            let cur_ind = matrix.list_of_inds[row_start_ind + j];
            if DEBUG_DETAILS {
                println!("Process {rank} of {size} getting index {cur_ind} in local row {i}");
            }
            if matrix.start_row <= cur_ind && cur_ind <= matrix.stop_row {
                matrix.list_of_inds[row_start_ind + j] -= matrix.start_row;
            } else {
                // Must find out if we have already set up this point
                if externals.contains_key(&cur_ind) {
                    // TODO: Negate index because stupid -- this may not work, since usized!
                    // matrix.list_of_inds[row_start_ind + j] = -(matrix.list_of_inds[row_start_ind + j] + 1);
                } else {
                    let num_external = num_external + 1;
                    externals.insert(cur_ind, num_external);
                    if num_external <= MAX_EXTERNAL {
                        matrix.external_index.push(cur_ind);
                        // TODO: Negate index because stupid -- this may not work, since usized!
                        // matrix.list_of_inds[row_start_ind + j] = -(matrix.list_of_inds[row_start_ind + j] + 1);
                    } else {
                        panic!("Must increase `MAX_EXTERNAL` from {MAX_EXTERNAL}");
                    }
                }
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Go through list of externals to find out which processors must be accessed.
    ////////////////////////////////////////////////////////////////////////////

    matrix.num_external = num_external;
    let mut tmp_buffer: Vec<usize> = vec![0; size];
    let mut global_index_offsets: Vec<usize> = Vec::with_capacity(size);

    tmp_buffer[rank] = matrix.start_row;

    // This call sends the start_row of each ith processor to the ith
    // entry of global_index_offset on all processors.
    // Thus, each processor know the range of indices owned by all
    // other processors.
    // Note:  There might be a better algorithm for doing this, but this
    //        will work...

    // MPI_Allreduce(tmp_buffer, global_index_offsets, size, MPI_INT, MPI_SUM, MPI_COMM_WORLD);
    // this populates globalindexoffsets

    world.all_reduce_into(
        &tmp_buffer,
        &mut global_index_offsets,
        SystemOperation::sum()
    );

    let mut external_processor = Vec::with_capacity(matrix.num_external);
    let mut new_external_processor: Vec<usize> = Vec::with_capacity(matrix.num_external);

    for i in 0..matrix.num_external {
        let cur_ind = matrix.external_index[i];
        for j in (0..size).rev() {
            if global_index_offsets[j] <= cur_ind {
                external_processor[i] = j;
                break;
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Sift through the external elements. For each newly encountered external
    // point assign it the next index in the sequence. Then look for other
    // external elements who are update by the same node and assign them the next
    // set of index numbers in the sequence (ie. elements updated by the same node
    // have consecutive indices).
    ////////////////////////////////////////////////////////////////////////////

    let count = matrix.local_nrow;
    matrix.external_local_index = vec![-1; num_external];

    ////////////////////////////////////////////////////////////////////////////
    //
    // Count the number of neighbors from which we receive information to update
    // our external elements. Additionally, fill the array tmp_neighbors in the
    // following way:
    //      tmp_neighbors[i] = 0   ==>  No external elements are updated by
    //                              processor i.
    //      tmp_neighbors[i] = x   ==>  (x-1)/size elements are updated from
    //                              processor i.
    //
    ////////////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////////
    //
    // Make a list of the neighbors that will send information to update our
    // external elements (in the order that we will receive this information).
    //
    /////////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////////
    //
    //  Compare the two lists. In most cases they should be the same.
    //  However, if they are not then add new entries to the recv list
    //  that are in the send list (but not already in the recv list).
    //
    /////////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////////
    // Start filling HPC_Sparse_Matrix struct
    /////////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////////
    //
    // Send each processor the global index list of the external elements in the
    // order that I will want to receive them when updating my external elements
    //
    /////////////////////////////////////////////////////////////////////////

    ///////////////////////////////////////////////////////////////////
    // Build "elements_to_send" list.  These are the x elements I own
    // that need to be sent to other processors.
    ///////////////////////////////////////////////////////////////////

    ////////////////
    // Finish up !!
    ////////////////

}