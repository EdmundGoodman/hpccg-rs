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
    // let mut residual: f64 = 0.0;
    // for i in 0..width {
    //     let diff = (actual[i] - expected[i]).abs();
    //     if diff > residual {
    //         residual = diff;
    //     }
    // }
    // residual
    // println!("actual={:?}, expected={:?}", actual, expected);
    actual
        .iter()
        .zip(expected.iter())
        .map(|(x, y)| (x - y).abs())
        // Need to account for f64 not being totally ordered (https://stackoverflow.com/a/50308360)
        .max_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN"))
        .unwrap_or(0.0)
}
