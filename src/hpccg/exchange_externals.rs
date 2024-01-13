use mpi::traits::*;

use super::mpi_universe::UNIVERSE;
use super::SparseMatrix;

pub fn exchange_externals(matrix: &SparseMatrix, vector: &[f64]) {
    let world = UNIVERSE.world();
    let size = world.size();
    let rank = world.rank();

    let local_nrow = matrix.local_nrow;
}