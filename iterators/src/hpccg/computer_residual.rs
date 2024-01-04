/// A method to compute the 1-norm difference between two vectors.
///
/// The 1-norm difference of two vectors is the largest absolute difference
/// between two values of the same index across the two vectors.
///
/// # Arguments
/// * `width` - The width of both input vectors.
/// * `actual` - The vector of actual values.
/// * `expected` - The vector of expected values.
pub fn compute_residual(actual: &Vec<f64>, expected: &Vec<f64>) -> f64 {
    // v1.iter().zip(v2.iter()).map(|(x, y)| (x-y).abs()).max().unwrap_or(0.0)
    actual.iter().zip(expected.iter())
        .map(|(x, y)| (x-y).abs())
        // Need to account for f64 not being totally ordered (https://stackoverflow.com/a/50308360)
        .max_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN"))
        .unwrap_or(0.0)
}

#[test]
fn test_compute_residual() {
    let vx = vec![1.0, 2.0, 3.0];
    let vy = vec![3.0, 2.0, 1.0];
    let r = compute_residual(&vx, &vy);
    assert_eq!(r, 2.0);
}