mod ddot;

use ddot::ddot_idiomatic;

fn main() {
    let vx = vec![1.0,2.0,3.0];
    let vy = vec![3.0,2.0,1.0];
    let r = ddot_idiomatic(&vx, &vy);
    println!("Idiomatic: {:?} . {:?} = {r}", vx, vy);
}
