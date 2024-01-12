use mpi::environment::Universe;
use mpi::traits::*;

use super::SparseMatrix;

pub fn exchange_externals(matrix: &SparseMatrix, vector: &[f64], universe: &Universe) {
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    let local_nrow = matrix.local_nrow;
}