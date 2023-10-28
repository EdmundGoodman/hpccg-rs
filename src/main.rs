pub mod ddot;
pub mod compute_residual;
pub mod waxpby;
pub mod hpc_sparse_matrix;
pub mod sparsemv;
pub mod mytimer;
mod hpccg;

use ddot::ddot_idiomatic;
use compute_residual::compute_residual_idiomatic;
use waxpby::waxpby_idiomatic;
use crate::mytimer::mytimer;

fn main() {
    let vx = vec![1.0,2.0,3.0];
    let vy = vec![3.0,2.0,1.0];
    let r = ddot_idiomatic(&vx, &vy);
    println!("{:?} . {:?} = {r}", vx, vy);

    let r = compute_residual_idiomatic(&vx, &vy);
    println!("residual({:?}, {:?}) = {r}", vx, vy);

    let alpha = 4.0;
    let beta = 5.0;
    let r = waxpby_idiomatic(alpha, &vx, beta, &vy);
    println!("waxpby({:?}, {:?}) = {:?}", vx, vy, r);
}
