use mpi::collective::SystemOperation;
use mpi::traits::*;

/// A method to compute the dot product of two vectors.
///
/// This function optimises caching by only accessing one of the vectors if both of the
/// input values point to the same vector.
///
/// # Arguments
/// * `_width` - The width of both input vectors.
/// * `lhs` - The first input vector.
/// * `rhs` - The second input vector.
pub fn ddot(_width: usize, lhs: &[f64], rhs: &[f64], world: &impl Communicator) -> f64 {
    let local_result: f64 = if std::ptr::eq(lhs, rhs) {
        lhs.iter().map(|x| x * x).sum()
    } else {
        lhs.iter().zip(rhs.iter()).map(|(x, y)| x * y).sum()
    };

    // TODO: Add another timer
    let mut global_result = 0.0;
    world.all_reduce_into(&local_result, &mut global_result, SystemOperation::sum());
    global_result
}
