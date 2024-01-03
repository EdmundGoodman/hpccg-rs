pub mod sparse_matrix;
mod ddot;

pub use sparse_matrix::SparseMatrix;
use ddot::ddot;

pub fn solver(matrix: SparseMatrix) -> f64 {
    let width = 3;
    let lhs = vec![1.0,2.0,3.0];
    let rhs = vec![3.0,2.0,1.0];
    ddot(width, &lhs, &rhs)
}
