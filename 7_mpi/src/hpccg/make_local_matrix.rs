use super::SparseMatrix;

use mpi::collective::SystemOperation;
use mpi::point_to_point::ReceiveFuture;
use mpi::traits::*;
use std::collections::HashMap;

const MAX_EXTERNAL: usize = 100000;
const MAX_NUM_MESSAGES: usize = 500;
const MAX_NUM_NEIGHBORS: usize = MAX_NUM_MESSAGES;

const DEBUG: bool = false;
const DEBUG_DETAILS: bool = false;

pub fn make_local_matrix(matrix: &mut SparseMatrix, world: &impl Communicator) {
    let mut externals: HashMap<usize, usize> = HashMap::new();
    let mut num_external: usize = 0;

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

    for i in 0..matrix.local_nrow {
        let row_start_ind = matrix.row_start_inds[i];
        for j in 0..matrix.nnz_in_row[i] {
            let cur_ind = matrix.list_of_inds[row_start_ind + j] as usize;
            if DEBUG_DETAILS {
                println!("Process {rank} of {size} getting index {cur_ind} in local row {i}");
            }
            if matrix.start_row <= cur_ind && cur_ind <= matrix.stop_row {
                matrix.list_of_inds[row_start_ind + j] -= matrix.start_row as i32;
            } else {
                // Must find out if we have already set up this point
                if externals.contains_key(&cur_ind) {
                    // Mark index as external by adding 1 and negating it
                    matrix.list_of_inds[row_start_ind + j] =
                        -(matrix.list_of_inds[row_start_ind + j] + 1);
                } else {
                    externals.insert(cur_ind, num_external);
                    num_external = num_external + 1;
                    if num_external <= MAX_EXTERNAL {
                        matrix.external_index.push(cur_ind);
                        // Mark index as external by adding 1 and negating it
                        matrix.list_of_inds[row_start_ind + j] =
                            -(matrix.list_of_inds[row_start_ind + j] + 1);
                    } else {
                        panic!("Must increase `MAX_EXTERNAL` from {MAX_EXTERNAL}");
                    }
                }
            }
        }
    }

    // println!("rank={}, num_external={}", rank, num_external);
    // println!("rank={}, externals={:?}", rank, &externals);

    // TODO: Add debug timer
    if DEBUG {
        println!("Processor {rank} of {size}: Number of external equations = {num_external}");
    }

    // TODO: Switch to multiple smaller function units called in sequence
    // let (externals, mut num_external) = scan_and_transform_local(matrix, world);

    ////////////////////////////////////////////////////////////////////////////
    // Go through list of externals to find out which processors must be accessed.
    ////////////////////////////////////////////////////////////////////////////

    let num_external = num_external; // TODO: Make immutable for debugging...
    matrix.num_external = num_external;
    let mut tmp_buffer: Vec<usize> = vec![0; size];
    // Needs to be of the correct size already! (not `Vec::with_capacity(size);`)
    let mut global_index_offsets: Vec<usize> = vec![0; size];

    tmp_buffer[rank] = matrix.start_row;

    // This call sends the start_row of each ith processor to the ith
    // entry of global_index_offset on all processors.
    // Thus, each processor know the range of indices owned by all
    // other processors.
    // Note:  There might be a better algorithm for doing this, but this
    //        will work...

    // MPI_Allreduce(tmp_buffer, global_index_offsets, size, MPI_INT, MPI_SUM, MPI_COMM_WORLD);
    world.all_reduce_into(
        &tmp_buffer,
        &mut global_index_offsets,
        SystemOperation::sum(),
    );

    // println!("rank={}, tmp_buffer={:?}", rank, &tmp_buffer);
    // println!(
    //     "rank={}, global_index_offsets={:?}",
    //     rank, &global_index_offsets
    // );
    // println!(
    //     "rank={}, matrix.external_index={:?}",
    //     rank, &matrix.external_index
    // );

    let mut external_processor = Vec::with_capacity(num_external);

    for i in 0..num_external {
        let cur_ind = matrix.external_index[i];
        for j in (0..size).rev() {
            if global_index_offsets[j] <= cur_ind {
                external_processor.push(j);
                break;
            }
        }
    }

    // println!(
    //     "rank={}, external_processor={:?}",
    //     rank, &external_processor
    // );

    ////////////////////////////////////////////////////////////////////////////
    // Sift through the external elements. For each newly encountered external
    // point assign it the next index in the sequence. Then look for other
    // external elements who are update by the same node and assign them the next
    // set of index numbers in the sequence (ie. elements updated by the same node
    // have consecutive indices).
    ////////////////////////////////////////////////////////////////////////////

    let mut count = matrix.local_nrow as i32;
    matrix.external_local_index = vec![-1; num_external];

    for i in 0..num_external {
        if matrix.external_local_index[i] == -1 {
            matrix.external_local_index[i] = count;
            count += 1;

            for j in i + 1..num_external {
                if external_processor[j] == external_processor[i] {
                    matrix.external_local_index[j] = count;
                    count += 1;
                }
            }
        }
    }

    // println!(
    //     "rank={}, matrix.external_local_index={:?}",
    //     rank, &matrix.external_local_index
    // );

    for i in 0..matrix.local_nrow {
        let row_start_ind = matrix.row_start_inds[i];
        for j in 0..matrix.nnz_in_row[i] {
            let poss_cur_ind = matrix.list_of_inds[row_start_ind + j];
            if poss_cur_ind < 0 {
                let cur_ind = (-poss_cur_ind - 1) as usize;
                // dbg!(externals[&cur_ind]);
                matrix.list_of_inds[row_start_ind + j] =
                    matrix.external_local_index[externals[&cur_ind]]
            }
        }
    }

    // println!(
    //     "rank={}, matrix.external_local_index={:?}",
    //     rank, &matrix.external_local_index
    // );

    let mut new_external_processor = vec![0usize; num_external];

    for i in 0..num_external {
        new_external_processor[matrix.external_local_index[i] as usize - matrix.local_nrow] =
            external_processor[i];
    }

    // println!(
    //     "rank={}, new_external_processor={:?}",
    //     rank, &new_external_processor
    // );

    if DEBUG_DETAILS {
        for i in 0..num_external {
            println!(
                "Processor {rank} of {size}: external processor[{i}] = {}",
                external_processor[i]
            );
            println!(
                "Processor {rank} of {size}: new external processor[{i}] = {}",
                new_external_processor[i]
            );
        }
    }

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

    let mut tmp_neighbors = vec![0; size];

    let mut num_recv_neighbors = 0;
    // let mut length = 1; // TODO: This is moved down and made not immutable?

    for i in 0..num_external {
        if (tmp_neighbors[new_external_processor[i]] == 0) {
            num_recv_neighbors += 1;
            tmp_neighbors[new_external_processor[i]] = 1;
        }
        tmp_neighbors[new_external_processor[i]] += size;
    }

    // println!("rank={}, num_recv_neighbors={}", rank, num_recv_neighbors);
    // println!("rank={}, tmp_neighbors={:?}", rank, &tmp_neighbors);

    // sum over all processors all the tmp_neighbors arrays //
    world.all_reduce_into(&tmp_neighbors, &mut tmp_buffer, SystemOperation::sum());

    // decode the combined 'tmp_neighbors' (stored in tmp_buffer)
    // array from all the processors
    let num_send_neighbors = tmp_buffer[rank] % size;
    // decode 'tmp_buffer[rank] to deduce total number of elements
    // we must send
    let total_to_be_sent = (tmp_buffer[rank] - num_send_neighbors) / size;

    // println!("rank={}, num_send_neighbors={}", rank, num_send_neighbors);
    // println!("rank={}, total_to_be_sent={}", rank, total_to_be_sent);
    // println!("rank={}, tmp_buffer={:?}", rank, &tmp_buffer);

    // Check to see if we have enough workspace allocated.  This could be
    // dynamically modified, but let's keep it simple for now...

    if num_send_neighbors > MAX_NUM_MESSAGES {
        panic!("Must increase `MAX_NUM_MESSAGES` to at least {num_send_neighbors}");
    }
    if total_to_be_sent > MAX_EXTERNAL {
        panic!("Must increase `MAX_EXTERNAL` to at least {total_to_be_sent}");
    }

    // TODO: Only needed in debug mode? Also add timers
    world.barrier();

    /////////////////////////////////////////////////////////////////////////
    //
    // Make a list of the neighbors that will send information to update our
    // external elements (in the order that we will receive this information).
    //
    /////////////////////////////////////////////////////////////////////////

    let mut recv_list = vec![];
    // TODO: This is a bug in the actual version! If n = 1, index out of bounds
    if num_external > 0 {
        recv_list.push(new_external_processor[0]);
    }
    for i in 1..num_external {
        if new_external_processor[i - 1] != new_external_processor[i] {
            recv_list.push(new_external_processor[i]);
        }
    }

    // Send a 0 length message to each of our recv neighbors
    let mut send_list = vec![0; num_send_neighbors];

    // println!("rank={}, recv_list={:?}", rank, &recv_list);
    // println!("rank={}, send_list={:?}", rank, &send_list);

    let mpi_my_tag = 99;
    // TODO: Are num_send_neighbors always the same?
    // TODO: Note that `send` cannot send `usize`, only `i32`
    // TODO: Make send/recv_list typed on Rank typedef?

    let placeholder_data = 1;
    let mut result_futures: Vec<ReceiveFuture<i32>> = vec![];
    for _ in 0..num_send_neighbors {
        result_futures.push(world.any_process().immediate_receive_with_tag(mpi_my_tag));
    }

    for i in 0..num_recv_neighbors {
        let _ = world
            .process_at_rank(recv_list[i] as i32)
            .send_with_tag(&placeholder_data, mpi_my_tag);
    }

    for (i, result_future) in result_futures.into_iter().enumerate() {
        // for i in 0..num_send_neighbors {
        let (msg, status) = result_future.get();
        assert_eq!(msg, placeholder_data);
        send_list[i] = status.source_rank() as usize;
    }

    // println!("rank={}, send_list={:?}", rank, &send_list);

    /////////////////////////////////////////////////////////////////////////
    //
    //  Compare the two lists. In most cases they should be the same.
    //  However, if they are not then add new entries to the recv list
    //  that are in the send list (but not already in the recv list).
    //
    /////////////////////////////////////////////////////////////////////////

    // println!("rank={}, num_recv_neighbors={}", rank, num_recv_neighbors);
    // println!("rank={}, recv_list={:?}", rank, &recv_list);

    for j in 0..num_send_neighbors {
        let mut found = 0;
        for i in 0..num_recv_neighbors {
            if recv_list[i] == send_list[j] {
                found = 1;
                break; // TODO: This is really an any pattern, could be an iterator
            }
        }

        if found == 0 {
            if DEBUG || DEBUG_DETAILS {
                println!(
                    "Processor {rank} of {size}: recv_list[{num_recv_neighbors}] = {}",
                    send_list[j]
                );
            }
            recv_list[num_recv_neighbors] = send_list[j];
            num_recv_neighbors += 1;
        }
    }

    // println!("rank={}, num_recv_neighbors={}", rank, num_recv_neighbors);
    // println!("rank={}, recv_list={:?}", rank, &recv_list);

    let num_send_neighbors = num_recv_neighbors;
    if num_send_neighbors > MAX_NUM_MESSAGES {
        // TODO: Is this wrong in the source code?!?!?!?
        panic!("Must increase `MAX_NUM_MESSAGES` from {MAX_NUM_MESSAGES}");
    }

    /////////////////////////////////////////////////////////////////////////
    // Start filling HPC_Sparse_Matrix struct
    /////////////////////////////////////////////////////////////////////////

    matrix.total_to_be_sent = total_to_be_sent;
    // matrix.elements_to_send =
    // let mut elements_to_send = vec![0; total_to_be_sent];
    // let mut elements_to_send = vec![];

    // Create 'new_external' which explicitly put the external elements in the
    // order given by 'external_local_index'

    let mut new_external = vec![0; num_external];
    for i in 0..num_external {
        new_external[matrix.external_local_index[i] as usize - matrix.local_nrow] =
            matrix.external_index[i];
    }

    // println!("rank={}, num_external={}", rank, num_external);
    // println!("rank={}, new_external={:?}", rank, &new_external);

    /////////////////////////////////////////////////////////////////////////
    //
    // Send each processor the global index list of the external elements in the
    // order that I will want to receive them when updating my external elements
    //
    /////////////////////////////////////////////////////////////////////////

    let mpi_my_tag = mpi_my_tag + 1;

    // First post receives
    let mut length_futures: Vec<ReceiveFuture<i32>> = vec![];
    for i in 0..num_send_neighbors {
        length_futures.push(
            world
                .process_at_rank(recv_list[i] as i32)
                .immediate_receive_with_tag(mpi_my_tag),
        );
    }

    // TODO: MAX_NUM_NEIGHBORS etc. can be replaced with runtime values?
    matrix.neighbors = Vec::with_capacity(MAX_NUM_NEIGHBORS);
    matrix.recv_length = Vec::with_capacity(MAX_NUM_NEIGHBORS);
    matrix.send_length = vec![0; num_recv_neighbors];

    let mut j = 0;
    for i in 0..num_recv_neighbors {
        let start = j;
        let mut newlength: usize = 0;

        // go through list of external elements until updating
        // processor changes
        while (j < num_external) && (new_external_processor[j] == recv_list[i]) {
            newlength += 1;
            j += 1;
            if j == num_external {
                break;
            }
        }

        matrix.recv_length.push(newlength);
        matrix.neighbors.push(recv_list[i]);

        let length = (j - start) as i32;
        let _ = world
            .process_at_rank(recv_list[i] as i32)
            .send_with_tag(&length, mpi_my_tag);
    }

    // print!("rank={}, lengths=[", rank);
    for (i, result_future) in length_futures.into_iter().enumerate() {
        let (msg, _) = result_future.get();
        // print!("{msg}, ");
        matrix.send_length[i] = msg as usize;
    }
    // println!("]");

    // println!("rank={}, matrix.neighbors={:?}", rank, &matrix.neighbors);
    // println!(
    //     "rank={}, matrix.recv_length={:?}",
    //     rank, &matrix.recv_length
    // );
    // println!(
    //     "rank={}, matrix.send_length={:?}",
    //     rank, &matrix.send_length
    // );

    ///////////////////////////////////////////////////////////////////
    // Build "elements_to_send" list.  These are the x elements I own
    // that need to be sent to other processors.
    ///////////////////////////////////////////////////////////////////

    let mpi_my_tag = mpi_my_tag + 1;

    // let mut result_slices: Vec<&mut Vec<i32>> = (0..num_recv_neighbors)
    //     .map(|i| vec![0; matrix.send_length[i]])
    //     .collect();

    let mut result_slices = vec![];
    for i in 0..num_recv_neighbors {
        let mut slice = vec![0; matrix.send_length[i]];
        result_slices.push(slice);
    }

    // println!("rank={}, result_slices={:?}", rank, &result_slices);

    let mut all_data_to_send = vec![];
    mpi::request::multiple_scope(num_recv_neighbors, |scope, coll| {
        for (i, slice) in result_slices.iter_mut().enumerate() {
            let rreq = world
                .process_at_rank(matrix.neighbors[i] as i32)
                // .any_process()
                .immediate_receive_into_with_tag(scope, slice, mpi_my_tag);
            coll.add(rreq);
        }

        let mut j = 0;
        for i in 0..num_recv_neighbors {
            let start = j;
            let mut newlength: usize = 0;

            // Go through list of external elements
            // until updating processor changes.  This is redundant, but
            // saves us from recording this information.

            while (j < num_external) && (new_external_processor[j] == recv_list[i]) {
                newlength += 1;
                j += 1;
                if j == num_external {
                    break;
                }
            }

            // TODO: The second send is somehow dropped
            let data_to_send = new_external[start..j]
                .iter()
                .map(|&x| x as i32)
                .collect::<Vec<i32>>();

            // println!(
            //     "rank={}, start={}, j={}, num_external={}, size={}, target={}, data={:?}",
            //     rank,
            //     start,
            //     j,
            //     num_external,
            //     new_external.len(),
            //     recv_list[i],
            //     data_to_send
            // );

            all_data_to_send.push(data_to_send);
            world
                .process_at_rank(recv_list[i] as i32)
                .send_with_tag(&all_data_to_send[i], mpi_my_tag);
        }

        while coll.incomplete() > 0 {
            let (request_index, status, _) = coll.wait_any().expect("MPI_Wait error");
            // println!(
            //     "rank={}, request_index={} | {:?}",
            //     rank, request_index, status
            // );
        }
    });

    // println!("rank={}, result_slices={:?}", rank, result_slices);

    // replace global indices by local indices
    for slice in result_slices.iter() {
        for &item in slice {
            let lhs = item as i32;
            let rhs = matrix.start_row as i32;
            matrix.elements_to_send.push(lhs - rhs);
        }
    }

    // println!("rank={}, elements_to_send={:?}", rank, &elements_to_send);

    // #[derive(Equivalence, PartialEq, Debug)]
    // struct MpiSendVec(Vec<i32>);

    // // let mut j = 0;
    // let mut elements_to_send_futures: Vec<ReceiveFuture<MpiSendVec>> = vec![];
    // for i in 0..num_recv_neighbors {
    //     elements_to_send_futures.push(
    //         world
    //             .process_at_rank(matrix.send_length[i] as i32)
    //             .immediate_receive_with_tag(mpi_my_tag),
    //     );
    //     // j += matrix.send_length[i];
    // }

    // let mut j = 0;
    // for i in 0..num_recv_neighbors {
    //     let start = j;
    //     let mut newlength: usize = 0;

    //     // Go through list of external elements
    //     // until updating processor changes.  This is redundant, but
    //     // saves us from recording this information.

    //     while (j < num_external) && (new_external_processor[j] == recv_list[i]) {
    //         newlength += 1;
    //         j += 1;
    //         if j == num_external {
    //             break;
    //         }
    //     }

    //     let data_to_send = MpiSendVec(
    //         new_external[start..j - start]
    //             .iter()
    //             .map(|&x| x as i32)
    //             .collect::<Vec<i32>>(),
    //     );
    //     let _ = world
    //         .process_at_rank(recv_list[i] as i32)
    //         .send_with_tag(&data_to_send, mpi_my_tag);
    // }

    // // receive from each neighbor the global index list of external elements

    // let mut j = 0;
    // print!("rank={}, lengths=[", rank);
    // for (i, element_to_send_future) in elements_to_send_futures.into_iter().enumerate() {
    //     let (msg, _) = element_to_send_future.get();
    //     print!("{:?}, ", msg);
    //     for k in 0..matrix.send_length[i] {
    //         matrix.elements_to_send[k + j] = msg[k];
    //     }
    //     j += matrix.send_length[i];
    // }
    // println!("]");

    // for i in 0..matrix.total_to_be_sent {
    //     elements_to_send[i] -= matrix.start_row;
    // }

    ////////////////
    // Finish up !!
    ////////////////

    matrix.num_send_neighbors = num_send_neighbors;
    matrix.local_ncol = matrix.local_nrow + num_external;
    matrix.send_buffer = vec![0.0; matrix.total_to_be_sent];

    // println!("{:?}", matrix);
}

//
//
//
//
//
//
//
//
//
//
//
//
//
//
//

// pub fn scan_and_transform_local(
//     matrix: &mut SparseMatrix,
//     world: &impl Communicator,
// ) -> (HashMap<usize, usize>, usize) {
//     let mut externals: HashMap<usize, usize> = HashMap::new();
//     let mut num_external: usize = 0;

//     let size = world.size() as usize;
//     let rank = world.rank() as usize;

//     ///////////////////////////////////////////
//     // Scan the indices and transform to local
//     ///////////////////////////////////////////

//     for i in 0..matrix.local_nrow {
//         let row_start_ind = matrix.row_start_inds[i];
//         for j in 0..matrix.nnz_in_row[i] {
//             let cur_ind = matrix.list_of_inds[row_start_ind + j] as usize;
//             if DEBUG_DETAILS {
//                 println!("Process {rank} of {size} getting index {cur_ind} in local row {i}");
//             }
//             if matrix.start_row <= cur_ind && cur_ind <= matrix.stop_row {
//                 matrix.list_of_inds[row_start_ind + j] -= matrix.start_row as i32;
//             } else {
//                 // Must find out if we have already set up this point
//                 if externals.contains_key(&cur_ind) {
//                     // Mark index as external by adding 1 and negating it
//                     matrix.list_of_inds[row_start_ind + j] =
//                         -(matrix.list_of_inds[row_start_ind + j] + 1);
//                 } else {
//                     num_external = num_external + 1;
//                     externals.insert(cur_ind, num_external);
//                     if num_external <= MAX_EXTERNAL {
//                         matrix.external_index.push(cur_ind);
//                         // Mark index as external by adding 1 and negating it
//                         matrix.list_of_inds[row_start_ind + j] =
//                             -(matrix.list_of_inds[row_start_ind + j] + 1);
//                     } else {
//                         panic!("Must increase `MAX_EXTERNAL` from {MAX_EXTERNAL}");
//                     }
//                 }
//             }
//         }
//     }

//     // TODO: Add debug timer
//     if DEBUG {
//         println!("Processor {rank} of {size}: Number of external equations = {num_external}");
//     }

//     (externals, num_external)
// }
