pub mod ddot;
pub mod compute_residual;
pub mod waxpby;
pub mod hpc_sparse_matrix;
pub mod sparsemv;
pub mod mytimer;
pub mod hpccg;

// use ddot::ddot_idiomatic;
// use compute_residual::compute_residual_idiomatic;
// use waxpby::waxpby_idiomatic;
// use crate::mytimer::mytimer;

use hpc_sparse_matrix::HpcSparseMatrix;
use hpccg::hpccg_direct;

fn main() {
//     let vx = vec![1.0,2.0,3.0];
//     let vy = vec![3.0,2.0,1.0];
//     let r = ddot_idiomatic(&vx, &vy);
//     println!("{:?} . {:?} = {r}", vx, vy);
//
//     let r = compute_residual_idiomatic(&vx, &vy);
//     println!("residual({:?}, {:?}) = {r}", vx, vy);
//
//     let alpha = 4.0;
//     let beta = 5.0;
//     let r = waxpby_idiomatic(alpha, &vx, beta, &vy);
//     println!("waxpby({:?}, {:?}) = {:?}", vx, vy, r);


    let (nx, ny, nz) = (2,2,2);
    let (matrix, x, b, xexact) = HpcSparseMatrix::generate_matrix(nx, ny, nz);

    let mut x = x.clone();
    let mut niters = 0;
    let mut normr = 0.0;
    let max_iter = 150;
    let tolerance = 0.0;
    let mut times: Vec<f64> = Vec::with_capacity(5);

    hpccg_direct(
        &matrix, &b, &mut x, max_iter, tolerance, &mut niters, &mut normr, &mut times
    )


}
