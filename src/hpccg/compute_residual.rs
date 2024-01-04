/// A method to compute the 1-norm difference between two vectors.
///
/// The 1-norm difference of two vectors is the largest absolute difference
/// between two values of the same index across the two vectors.
///
/// # Arguments
/// * `width` - The width of both input vectors.
/// * `actual` - The vector of actual values.
/// * `expected` - The vector of expected values.
pub fn compute_residual(width: usize, actual: &[f64], expected: &[f64]) -> f64 {
    let mut residual: f64 = 0.0;
    for i in 0..width {
        let diff = (actual[i] - expected[i]).abs();
        if diff > residual {
            residual = diff;
        }
    }
    residual

}

#[test]
fn test_compute_residual() {
    let width = 3;
    let vx = vec![1.0, 2.0, 3.0];
    let vy = vec![3.0, 2.0, 1.0];
    let r = compute_residual(width, &vx, &vy);
    assert_eq!(r, 2.0);
}