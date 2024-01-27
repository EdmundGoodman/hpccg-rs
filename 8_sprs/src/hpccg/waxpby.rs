use ndarray::Array1;

/// A function to compute the sum of two scaled vectors.
///
/// # Arguments
/// * `alpha` - The scaling factor for the first vector.
/// * `x` - The first input vector.
/// * `beta` - The scaling factor for the second vector.
/// * `y` - The second input vector.
pub fn waxpby(alpha: f64, x: &Array1<f64>, beta: f64, y: &Array1<f64>) -> Array1<f64> {
    if alpha == 1.0 {
        x + (y * beta)
    } else if beta == 1.0 {
        (x * alpha) + y
    } else {
        (x * alpha) + (y * beta)
    }
}
