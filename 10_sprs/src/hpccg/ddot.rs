use sprs::CsVec;

/// A method to compute the dot product of two vectors.
///
/// This function optimises caching by only accessing one of the vectors if both of the
/// input values point to the same vector.
///
/// # Arguments
/// * `_width` - The width of both input vectors.
/// * `lhs` - The first input vector.
/// * `rhs` - The second input vector.
pub fn ddot(_width: usize, lhs: &CsVec<f64>, rhs: &CsVec<f64>) -> f64 {
    lhs.dot(rhs)
}

#[test]
fn test_ddot() {
    let width = 3;
    let lhs = vec![1.0, 2.0, 3.0];
    let rhs = vec![3.0, 2.0, 1.0];
    let result = ddot(width, &lhs, &rhs);
    assert_eq!(result, 10.0);
    let result = ddot(width, &lhs, &lhs);
    assert_eq!(result, 14.0);
}
