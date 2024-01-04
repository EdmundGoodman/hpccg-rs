/// A method to compute the dot product of two vectors.
///
/// This function optimises caching by only accessing one of the vectors if both of the
/// input values point to the same vector.
///
/// # Arguments
/// * `width` - The width of both input vectors.
/// * `lhs` - The first input vector.
/// * `rhs` - The second input vector.
pub fn ddot(lhs: &Vec<f64>, rhs: &Vec<f64>) -> f64 {
    if std::ptr::eq(lhs, rhs) {
        lhs.iter().map(|x| x * x).sum()
    } else {
        lhs.iter().zip(rhs.iter())
            .map(|(x, y)| x * y).sum()
    }
}

#[test]
fn test_ddot() {
    let width = 3;
    let lhs = vec![1.0,2.0,3.0];
    let rhs = vec![3.0,2.0,1.0];
    let result = ddot(&lhs, &rhs);
    assert_eq!(result, 10.0);
    let result = ddot(&lhs, &lhs);
    assert_eq!(result, 14.0);
}