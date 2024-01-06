/// A method to compute the dot product of two vectors.
///
/// This function optimises caching by only accessing one of the vectors if both of the
/// input values point to the same vector.
///
/// # Arguments
/// * `width` - The width of both input vectors.
/// * `lhs` - The first input vector.
/// * `rhs` - The second input vector.
pub fn ddot(width: usize, lhs: &[f64], rhs: &[f64]) -> f64 {
    let mut result: f64 = 0.0;
    if std::ptr::eq(lhs, rhs) {
        for i in 0..width {
            result += lhs[i] * lhs[i];
        }
    } else {
        for i in 0..width {
            result += lhs[i] * rhs[i];
        }
    }
    result
}

#[test]
fn test_ddot() {
    let width = 3;
    let lhs = vec![1.0, 2.0, 3.0];
    let rhs = vec![3.0, 2.0, 1.0];
    let result = ddot(width, &lhs, &rhs);
    assert_eq!(result, 10.0);
    let result = ddot(width, &lhs, &lhs);
    assert_eq!(result, 14.0);
}





use autocxx::prelude::*;
use autocxx::c_int;

include_cpp! {
    #include "ddot.hpp"
    safety!(unsafe_ffi)
    generate!("ddot")
}

#[test]
fn test_ddot_cpp() {
    let n = c_int(3);
    let mut x = vec![ 1.0, 2.0, 3.0 ];
    let mut y = vec![ 3.0, 2.0, 1.0 ];
    let mut result = 0.0;
    let expected = 10.0;
    unsafe {
        ffi::ddot(n, x.as_mut_ptr(), y.as_mut_ptr(), &mut result);
    }
    assert_eq!(result, expected);
}
