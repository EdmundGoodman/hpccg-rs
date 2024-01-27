pub mod compute_residual;
mod ddot;
mod mytimer;
pub mod sparse_matrix;
mod sparsmv;
mod waxpby;

pub use compute_residual::compute_residual;
use ddot::ddot;
use mytimer::mytimer;
pub use sparse_matrix::generate_matrix;
use sparsmv::sparsemv;
use waxpby::waxpby;

use ndarray::Array1;
use sprs::CsMat;

/// Store the start time for a code section.
fn tick(t0: &mut f64) {
    *t0 = mytimer();
}

/// Increment the total time for a code section by the most recent interval.
fn tock(t0: &f64, t: &mut f64) {
    *t += mytimer() - t0;
}

/// A method to computer the approximate solution to `Ax = b`
///
/// # Arguments
/// * `A` - The input sparse matrix.
/// * `b` - The known right hand side vector.
/// * `x` - The current approximate solution, which starts as the initial guess.
/// * `max_iter` - The maximum number of iterations to perform.
/// * `tolerance` - The value the residual needs to be less than for convergence (how "good" of a
///                 solution do we need).
///
/// # Return values
/// * `result` - The approximate result at the end of the solver loop
/// * `iterations` - The number of iterations for which the solver ran
/// * `normr` - The residual difference between the current approximate solution and the exact
///             solution.
/// * `times` - An array of times spent for each operation (ddot/waxpby/sparse_mv/total).
#[allow(non_snake_case, unused_assignments, unused_mut)]
pub fn solver(
    A: &CsMat<f64>,
    b: Array1<f64>,
    x: Array1<f64>,
    max_iterations: i32,
    tolerance: f64,
) -> (Array1<f64>, i32, f64, Vec<f64>) {
    let t_begin: f64 = mytimer();
    let mut t_total: f64 = 0.0;
    let mut t_ddot: f64 = 0.0;
    let mut t_waxpby: f64 = 0.0;
    let mut t_sparsemv: f64 = 0.0;
    let mut t_mpi_allreduce: f64 = 0.0;

    let mut r: Array1<f64>;
    let mut p: Array1<f64>;
    let mut Ap: Array1<f64>;

    let mut result = x.to_owned();
    let mut iteration = 0;

    let mut normr = 0.0;
    let mut rtrans: f64 = 0.0;
    let mut oldrtrans: f64 = 0.0;

    let print_freq = (max_iterations / 10).max(1).min(50);

    // `p` is of length `ncols`, so copy `x` to `p` for sparse matrix-vector operation
    tick(&mut t_total);
    p = waxpby(1.0, &result, 0.0, &b);
    tock(&t_total, &mut t_waxpby);

    tick(&mut t_total);
    Ap = sparsemv(A, &p);
    tock(&t_total, &mut t_sparsemv);

    tick(&mut t_total);
    r = waxpby(1.0, &b, -1.0, &Ap);
    tock(&t_total, &mut t_waxpby);

    tick(&mut t_total);
    rtrans = ddot(&r, &r);
    tock(&t_total, &mut t_ddot);

    normr = rtrans.sqrt();

    println!("Initial Residual = {normr:+.5e}");

    for k in 1..max_iterations {
        if normr <= tolerance {
            break;
        }

        if k == 1 {
            tick(&mut t_total);
            p = waxpby(1.0, &r, 0.0, &r);
            tock(&t_total, &mut t_waxpby);
        } else {
            oldrtrans = rtrans;
            tick(&mut t_total);
            rtrans = ddot(&r, &r);
            tock(&t_total, &mut t_ddot);
            let beta = rtrans / oldrtrans;
            tick(&mut t_total);
            p = waxpby(1.0, &r, beta, &p);
            tock(&t_total, &mut t_waxpby);
        }

        normr = rtrans.sqrt();
        if k % print_freq == 0 || k + 1 == max_iterations {
            println!("Iteration = {k} , Residual = {normr:+.5e}");
        }

        tick(&mut t_total);
        Ap = sparsemv(A, &p);
        tock(&t_total, &mut t_sparsemv);

        tick(&mut t_total);
        let alpha = ddot(&p, &Ap);
        tock(&t_total, &mut t_ddot);

        let alpha = rtrans / alpha;
        tick(&mut t_total);
        result = waxpby(1.0, &result, alpha, &p);
        r = waxpby(1.0, &r, -alpha, &Ap);
        tock(&t_total, &mut t_waxpby);
        iteration = k;
    }

    (
        result,
        iteration,
        normr,
        vec![
            mytimer() - t_begin,
            t_ddot,
            t_waxpby,
            t_sparsemv,
            t_mpi_allreduce,
        ],
    )
}

// #[test]
// fn test_solver() {
//     let (nx, ny, nz) = (5, 5, 5);
//     let (matrix, guess, rhs, exact) = generate_matrix(nx, ny, nz);
//     let max_iter = 150;
//     let tolerance = 5e-40;
//     let (result, iterations, normr, _) = solver(&matrix, &rhs, &guess, max_iter, tolerance);
//     let residual = compute_residual(matrix.rows(), &result, &exact);
//     assert!(normr < tolerance);
//     assert!(iterations < max_iter);
//     assert!(residual < 1e-15);
//     for (actual, expected) in result.iter().zip(exact) {
//         assert!((expected - actual).abs() < 1e-5);
//     }
// }
