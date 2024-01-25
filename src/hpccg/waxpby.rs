use mpi::traits::*;

/// A function to compute the sum of two scaled vectors.
///
/// # Arguments
/// * `width` - The width of both input vectors.
/// * `alpha` - The scaling factor for the first vector.
/// * `x` - The first input vector.
/// * `beta` - The scaling factor for the second vector.
/// * `y` - The second input vector.
pub fn waxpby(
    width: usize,
    alpha: f64,
    x: &[f64],
    beta: f64,
    y: &[f64],
    world: &impl Communicator,
) -> Vec<f64> {
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
    let mut result = Vec::with_capacity(width);
    debug_assert!(x.len() == width && y.len() == width);
    if alpha == 1.0 {
        for i in 0..width {
            result.push(unsafe { x.get_unchecked(i) + beta * y.get_unchecked(i) });
            // if world.rank() == 0 {
            //     println!("w[{}]={}", i, result[i]);
            // }
            // result.push(x[i] + beta * y[i]);
        }
    } else if beta == 1.0 {
        for i in 0..width {
            result.push(unsafe { alpha * x.get_unchecked(i) + y.get_unchecked(i) });
            // result.push(alpha * x[i] + y[i]);
        }
    } else {
        for i in 0..width {
            result.push(unsafe { alpha * x.get_unchecked(i) + beta * y.get_unchecked(i) });
            // result.push(alpha * x[i] + beta * y[i]);
        }
    }
    result
}
