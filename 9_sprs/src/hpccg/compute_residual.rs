#[allow(unused_imports)]
use ndarray::{array, Array1};

/// A method to compute the 1-norm difference between two vectors.
///
/// The 1-norm difference of two vectors is the largest absolute difference
/// between two values of the same index across the two vectors.
///
/// # Arguments
/// * `actual` - The vector of actual values.
/// * `expected` - The vector of expected values.
pub fn compute_residual(actual: &Array1<f64>, expected: &Array1<f64>) -> f64 {
    actual
        .iter()
        .zip(expected.iter())
        .map(|(x, y)| (x - y).abs())
        // Need to account for f64 not being totally ordered (https://stackoverflow.com/a/50308360)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less))
        .unwrap_or(f64::NAN)
}

#[test]
fn test_compute_residual() {
    let vx = array![1.0, 2.0, 3.0];
    let vy = array![3.0, 2.0, 1.0];
    let r = compute_residual(&vx, &vy);
    assert_eq!(r, 2.0);
}
