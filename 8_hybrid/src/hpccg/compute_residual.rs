use rayon::prelude::*;
use std::cmp::Ordering;

/// A method to compute the 1-norm difference between two vectors.
///
/// The 1-norm difference of two vectors is the largest absolute difference
/// between two values of the same index across the two vectors.
///
/// # Arguments
/// * `_width` - The width of both input vectors.
/// * `actual` - The vector of actual values.
/// * `expected` - The vector of expected values.
pub fn compute_residual(_width: usize, actual: &[f64], expected: &[f64]) -> f64 {
    actual
        .par_iter()
        .zip(expected.par_iter())
        .map(|(x, y)| (x - y).abs())
        // Need to account for f64 not being totally ordered (https://stackoverflow.com/a/50308360)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less))
        .unwrap_or(f64::NAN)
}