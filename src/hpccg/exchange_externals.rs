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
    vector: &mut [f64],
    world: &impl Communicator,
) {
    // let size = world.size();
    // let rank = world.rank();

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
