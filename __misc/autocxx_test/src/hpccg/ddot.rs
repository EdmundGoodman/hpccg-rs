use crate::cpp_ffi::ddot as ffi_ddot;
use autocxx::c_int;
use std::pin::Pin;

const TEST_RUST: bool = false;

// include_cpp! {
//     #include "ddot.hpp"
//     safety!(unsafe_ffi)
//     generate!("ddot")
// }

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

    let result = if TEST_RUST {
        ddot(width, &lhs, &rhs)
    } else {
        let mut result = 0.0;
        let mut tmp = 0.0;
        unsafe {
            ffi_ddot(
                c_int(width as i32),
                lhs.as_ptr(),
                rhs.as_ptr(),
                &mut result,
                Pin::new(&mut tmp),
            );
        }
        result
    };
    assert_eq!(result, 10.0);

    let result = if TEST_RUST {
        ddot(width, &lhs, &lhs)
    } else {
        let mut result = 0.0;
        let mut tmp = 0.0;
        unsafe {
            ffi_ddot(
                c_int(width as i32),
                lhs.clone().as_mut_ptr(),
                lhs.clone().as_mut_ptr(),
                &mut result,
                Pin::new(&mut tmp),
            );
        }
        result
    };
    assert_eq!(result, 14.0);
}
