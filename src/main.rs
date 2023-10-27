mod ddot;
mod compute_residual;

use ddot::ddot_idiomatic;
use compute_residual::compute_residual_idiomatic;

fn main() {
    let vx = vec![1.0,2.0,3.0];
    let vy = vec![3.0,2.0,1.0];
    let r = ddot_idiomatic(&vx, &vy);
    println!("{:?} . {:?} = {r}", vx, vy);

    let vx = vec![1.0,2.0,3.0];
    let vy = vec![3.0,2.0,1.0];
    let r = compute_residual_idiomatic(&vx, &vy);
    println!("residual({:?}, {:?}) = {r}", vx, vy);
}
