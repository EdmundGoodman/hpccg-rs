use mpi::traits::*;

use super::SparseMatrix;

pub fn exchange_externals(matrix: &SparseMatrix, _vector: &[f64], world: &impl Communicator) {
    let _size = world.size();
    let _rank = world.rank();
    let _local_nrow = matrix.local_nrow;
}
