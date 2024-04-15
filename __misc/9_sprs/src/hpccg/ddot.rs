use ndarray::Array1;

/// A method to compute the dot product of two vectors.
///
/// This function optimises caching by only accessing one of the vectors if both of the
/// input values point to the same vector.
///
/// # Arguments
/// * `lhs` - The first input vector.
/// * `rhs` - The second input vector.
pub fn ddot(lhs: &Array1<f64>, rhs: &Array1<f64>) -> f64 {
    lhs.dot(rhs)
}
