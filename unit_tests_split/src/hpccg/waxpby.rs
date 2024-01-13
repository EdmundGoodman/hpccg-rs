use rayon::prelude::*;

/// A function to compute the sum of two scaled vectors.
///
/// # Arguments
/// * `width` - The width of both input vectors.
/// * `alpha` - The scaling factor for the first vector.
/// * `x` - The first input vector.
/// * `beta` - The scaling factor for the second vector.
/// * `y` - The second input vector.
pub fn waxpby(_width: usize, alpha: f64, x: &[f64], beta: f64, y: &[f64]) -> Vec<f64> {
    if alpha == 1.0 {
        x.par_iter()
            .zip(y.par_iter())
            .map(|(x, y)| x + beta * y)
            .collect()
    } else if beta == 1.0 {
        x.par_iter()
            .zip(y.par_iter())
            .map(|(x, y)| alpha * x + y)
            .collect()
    } else {
        x.par_iter()
            .zip(y.par_iter())
            .map(|(x, y)| alpha * x + beta * y)
            .collect()
    }
}
