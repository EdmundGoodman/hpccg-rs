use autocxx::prelude::*;
use autocxx::c_int;

include_cpp! {
    #include "ddot.hpp"
    safety!(unsafe_ffi)
    generate!("ddot")
}

fn main() {
    let n = c_int(3);
    let mut x: [f64; 3] = [ 1.0, 2.0, 3.0 ];
    let mut y: [f64; 3] = [ 3.0, 2.0, 1.0 ];
    let mut result: f64 = 0.0; //unsafe { std::mem::uninitialized() };
    unsafe {
        ffi::ddot(n, x.as_mut_ptr(), y.as_mut_ptr(), &mut result);
    }
    println!("{:?} dot product {:?} = {}", x, y, result);
}
