/// A data structure representing a sparse matrix mesh
///
/// # Fields
/// * `start_row` - The row to start generating the matrix from (always `0` in serial mode)
/// * `stop_row` - The row to stop generating the matrix at (always `x*y*z-1` in serial mode)
#[derive(Debug)]
#[allow(dead_code)]
pub struct SparseMatrix {
    pub start_row: i32,
    pub stop_row: i32,
}
