use mpi::request::Request;
use mpi::traits::*;
use mpi::Rank;

use super::SparseMatrix;

/// A method to exchange external data between MPI processes.
///
/// # Arguments
/// * `matrix` - The sparse matrix currently being computed.
/// * `vector` - The data to be sent.
/// * `world` - The MPI world to communicate over.
pub fn exchange_externals(
    matrix: &mut SparseMatrix,
    vector: &mut Vec<f64>,
    world: &impl Communicator,
) {
    let size = world.size();
    let rank = world.rank();

    let mpi_my_tag = 99;

    let mut x_externals = vec![];
    for i in 0..matrix.num_send_neighbors {
        let mut slice = vec![0.0; matrix.recv_length[i]];
        x_externals.push(slice);
    }

    // Fill up send buffer
    for i in 0..matrix.total_to_be_sent {
        matrix.send_buffer[i] = vector[matrix.elements_to_send[i] as usize];
    }

    mpi::request::multiple_scope(matrix.num_send_neighbors, |scope, coll| {
        // Post receives first
        for (i, x_external) in x_externals.iter_mut().enumerate() {
            let rreq = world
                .process_at_rank(matrix.neighbors[i] as i32)
                .immediate_receive_into_with_tag(scope, x_external, mpi_my_tag);
            coll.add(rreq);
        }
        // x_externals will then be flattened and pushed to vector at the end...

        // Send to each neighbor
        let mut start = 0;
        for i in 0..matrix.num_send_neighbors {
            world
                .process_at_rank(matrix.neighbors[i] as i32)
                .send_with_tag(
                    &matrix.send_buffer[start..start + matrix.send_length[i]],
                    mpi_my_tag,
                );

            // println!(
            //     "rank={}, target={}, data={:?}",
            //     rank,
            //     matrix.neighbors[i],
            //     &matrix.send_buffer[start..start + matrix.send_length[i]]
            // );
            start += matrix.send_length[i];
        }

        while coll.incomplete() > 0 {
            let (request_index, status, _) = coll.wait_any().expect("MPI_Wait error");
        }
    });

    // println!(
    //     "rank={}, vector.len={}, x_externals.len={}, local_nrow={}",
    //     rank,
    //     vector.len(),
    //     x_externals[0].len(),
    //     matrix.local_nrow
    // );

    // println!("local={} , global={}", matrix.local_nrow, matrix.total_nrow);
    // let mut i = matrix.local_nrow - 1;
    for x_external in x_externals.iter() {
        for &item in x_external {
            // println!("rank={}, i={}, length={}", rank, i, vector.len());
            // vector[i] = item;
            // i += 1;
            vector.push(item);
        }
    }
    assert_eq!(vector.len(), matrix.local_ncol);

    // panic!("quit");

    // for slice in result_slices.iter() {
    //     for &item in slice {
    //         let lhs = item as i32;
    //         let rhs = matrix.start_row as i32;
    //         matrix.elements_to_send.push(lhs - rhs);
    //     }
    // }

    // ===== //
    // let num_external = 0;
    // let mut x_external: Vec<f64> = vec![];

    // mpi::request::scope(|scope| {
    //     let reqs = vec![];
    //     for neighbor in matrix.neighbors.iter() {
    //         let mut recv = vec![];
    //         let req = world
    //             .process_at_rank(*neighbor as Rank)
    //             .immediate_receive_into(scope, &mut recv);
    //         x_external.append(&mut (recv.clone()));
    //         reqs.push(req);
    //     }

    //     // Fill up the send buffer
    //     for i in 0..matrix.total_to_be_sent {
    //         matrix.send_buffer[i] = vector[matrix.elements_to_send[i]];
    //     }
    //     // Send to each neighbor
    //     for neighbor in matrix.neighbors {
    //         world
    //             .process_at_rank(neighbor as Rank)
    //             .send(&matrix.send_buffer);
    //     }
    //     // Complete the reads issued above
    //     for req in reqs {
    //         req.wait();
    //     }
    // });
}
