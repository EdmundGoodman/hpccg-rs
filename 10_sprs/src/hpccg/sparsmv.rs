use ndarray::Array1;
use sprs::CsMat;

/// Sparse matrix-vector multiplication
///
/// # Arguments
/// * `matrix` - A representation of a sparse matrix.
/// * `vector` - The input vector to multiply the sparse matrix by.
pub fn sparsemv(matrix: &CsMat<f64>, vector: &Array1<f64>) -> Array1<f64> {
    matrix * vector
}
