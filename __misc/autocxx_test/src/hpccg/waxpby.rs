use crate::cpp_ffi::waxpby as ffi_waxpby;
use autocxx::c_int;

const TEST_RUST: bool = false;

/// A function to compute the sum of two scaled vectors.
///
/// # Arguments
/// * `width` - The width of both input vectors.
/// * `alpha` - The scaling factor for the first vector.
/// * `x` - The first input vector.
/// * `beta` - The scaling factor for the second vector.
/// * `y` - The second input vector.
pub fn waxpby(width: usize, alpha: f64, x: &[f64], beta: f64, y: &[f64]) -> Vec<f64> {
    let mut result = Vec::with_capacity(width);
    if alpha == 1.0 {
        for i in 0..width {
            result.push(x[i] + beta * y[i]);
            // result[i] = x[i] + beta * y[i];
        }
    } else if beta == 1.0 {
        for i in 0..width {
            result.push(alpha * x[i] + y[i]);
        }
    } else {
        for i in 0..width {
            result.push(alpha * x[i] + beta * y[i]);
        }
    }
    result
}

#[test]
fn test_waxpby() {
    let width = 3;
    let vx = vec![1.0, 2.0, 3.0];
    let vy = vec![3.0, 2.0, 1.0];
    let alpha = 4.0;
    let beta = 5.0;

    let result = if TEST_RUST {
        waxpby(width, alpha, &vx, beta, &vy)
    } else {
        let mut result = vec![0.0; 3];
        unsafe {
            ffi_waxpby(
                c_int(width as i32),
                alpha,
                vx.as_ptr(),
                beta,
                vy.as_ptr(),
                result.as_mut_ptr(),
            );
        }
        result
    };
    assert_eq!(result, vec![19.0, 18.0, 17.0]);

    let alpha = 1.0;
    let result = if TEST_RUST {
        waxpby(width, alpha, &vx, beta, &vy)
    } else {
        let mut result = vec![0.0; 3];
        unsafe {
            ffi_waxpby(
                c_int(width as i32),
                alpha,
                vx.as_ptr(),
                beta,
                vy.as_ptr(),
                result.as_mut_ptr(),
            );
        }
        result
    };
    assert_eq!(result, vec![16.0, 12.0, 8.0]);

    let alpha = 4.0;
    let beta = 1.0;
    let result = if TEST_RUST {
        waxpby(width, alpha, &vx, beta, &vy)
    } else {
        let mut result = vec![0.0; 3];
        unsafe {
            ffi_waxpby(
                c_int(width as i32),
                alpha,
                vx.as_ptr(),
                beta,
                vy.as_ptr(),
                result.as_mut_ptr(),
            );
        }
        result
    };
    assert_eq!(result, vec![7.0, 10.0, 13.0]);
}