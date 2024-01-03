pub mod ddot;
pub mod compute_residual;
pub mod waxpby;
pub mod hpc_sparse_matrix;
pub mod sparsemv;
pub mod mytimer;
pub mod hpccg;

use hpc_sparse_matrix::HpcSparseMatrix;
use hpccg::hpccg_direct;


/// The driver code for the calculating the conjugate gradient.
///
/// First,the progam generatess the matrix, right hand side vector,
/// exact solution vector, and an initial guess. Then, it calls the
/// HPCCG conjugate gradient solver on the matrix and associated data.
/// Finally, it print the result of the solver, and information about
/// the performance of the computation.
fn main() {
    let (nx, ny, nz) = (25,25,25);
    let (matrix, x, b, xexact) = HpcSparseMatrix::generate_matrix(nx, ny, nz);

    let mut x = x.clone();
    let mut niters = 0;
    let mut normr = 0.0;
    let max_iter = 150;
    let tolerance = 0.0;
    let mut times: Vec<f64> = Vec::with_capacity(5);

    hpccg_direct(
        &matrix, &b, &mut x, max_iter, tolerance, &mut niters, &mut normr, &mut times
    );

    let fniters = niters;
    let fnrow = matrix.total_nrow;
    let fnnz = matrix.total_nnz;
    let fnops_ddot = fniters*4*fnrow;
    let fnops_waxpby = fniters*6*fnrow;
    let fnops_sparsemv = fniters*2*fnnz;
    let fnops = fnops_ddot+fnops_waxpby+fnops_sparsemv;

    println!("Mini-Application Name: hpccg");
    println!("Mini-Application Version: 1.0");
    println!("Parallelism:\n  MPI not enabled:\n  OpenMP not enabled:");
    println!("Dimensions:\n  nx: {nx}\n  ny: {ny}\n  nz: {nz}");
    println!("Number of iterations: {niters}");
    println!("Final residual: {normr:.5e}");
    println!("#********** Performance Summary (times in sec) ***********");
    println!("Time Summary:");
    println!("  Total: {:.4}", times[0]);
    println!("  DDOT: {:.4}", times[1]);
    println!("  WAXPBY: {:.4}", times[2]);
    println!("  SPARSEMV: {:.4}", times[3]);
    println!("FLOPS Summary:");
    println!("  Total: {fnops:.4}");
    println!("  DDOT: {fnops_ddot:.4}");
    println!("  WAXPBY: {fnops_waxpby:.4}");
    println!("  SPARSEMV: {fnops_sparsemv:.4}");
    println!("MFLOPS Summary:");
    println!("  Total: {:.4}", (fnops as f64)/times[0]/1.0e6);
    println!("  DDOT: {:.4}", (fnops_ddot as f64)/times[1]/1.0e6);
    println!("  WAXPBY: {:.4}", (fnops_waxpby as f64)/times[2]/1.0e6);
    println!("  SPARSEMV: {:.4}", (fnops_sparsemv as f64)/times[3]/1.0e6);
}
