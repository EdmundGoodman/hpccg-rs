use rayon::prelude::*;

/// A method to compute the dot product of two vectors.
///
/// This function optimises caching by only accessing one of the vectors if both of the
/// input values point to the same vector.
///
/// # Arguments
/// * `_width` - The width of both input vectors.
/// * `lhs` - The first input vector.
/// * `rhs` - The second input vector.
pub fn ddot(_width: usize, lhs: &[f64], rhs: &[f64]) -> f64 {
    if std::ptr::eq(lhs, rhs) {
        lhs.par_iter().map(|x| x * x).sum()
    } else {
        lhs.par_iter().zip(rhs.par_iter()).map(|(x, y)| x * y).sum()
    }
}
