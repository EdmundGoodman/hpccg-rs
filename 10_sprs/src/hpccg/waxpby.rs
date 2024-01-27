use sprs::CsVec;

/// A function to compute the sum of two scaled vectors.
///
/// # Arguments
/// * `alpha` - The scaling factor for the first vector.
/// * `x` - The first input vector.
/// * `beta` - The scaling factor for the second vector.
/// * `y` - The second input vector.
pub fn waxpby(alpha: f64, x: &CsVec<f64>, beta: f64, y: &CsVec<f64>) -> CsVec<f64> {
    x.map(|i| i * alpha) + y.map(|i| i * beta)
    // (x * alpha) + (beta * y)
    // if alpha == 1.0 {
    //     x.iter().zip(y.iter()).map(|(x, y)| x + beta * y).collect()
    // } else if beta == 1.0 {
    //     x.iter().zip(y.iter()).map(|(x, y)| alpha * x + y).collect()
    // } else {
    //     x.iter()
    //         .zip(y.iter())
    //         .map(|(x, y)| alpha * x + beta * y)
    //         .collect()
    // }
}

// #[test]
// fn test_waxpby() {
//     let width = 3;
//     let vx = vec![1.0, 2.0, 3.0];
//     let vy = vec![3.0, 2.0, 1.0];
//     let alpha = 4.0;
//     let beta = 5.0;
//     let result = waxpby(width, alpha, &vx, beta, &vy);
//     assert_eq!(result, vec![19.0, 18.0, 17.0]);
//     let alpha = 1.0;
//     let result = waxpby(width, alpha, &vx, beta, &vy);
//     assert_eq!(result, vec![16.0, 12.0, 8.0]);
//     let alpha = 4.0;
//     let beta = 1.0;
//     let result = waxpby(width, alpha, &vx, beta, &vy);
//     assert_eq!(result, vec![7.0, 10.0, 13.0]);
// }
