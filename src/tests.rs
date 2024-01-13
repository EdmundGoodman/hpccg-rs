#[cfg(test)]
mod tests {
    use crate::hpccg::hpccg_internals::{
        compute_residual,
        ddot,
        mytimer,
        SparseMatrix,
        sparsemv,
        waxpby
    };

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // fn test_ddot() {
    //     let width = 3;
    //     let lhs = vec![1.0, 2.0, 3.0];
    //     let rhs = vec![3.0, 2.0, 1.0];
    //     let result = ddot(width, &lhs, &rhs);
    //     assert_eq!(result, 10.0);
    //     let result = ddot(width, &lhs, &lhs);
    //     assert_eq!(result, 14.0);
    // }
    //
    // #[test]
    // fn integration() {
    //     let (nx, ny, nz) = (5, 5, 5);
    //     let (matrix, guess, rhs, exact) = SparseMatrix::generate_matrix(nx, ny, nz);
    //     let max_iter = 150;
    //     let tolerance = 5e-40;
    //     let (result, iterations, normr, _) = solver(&matrix, &rhs, &guess, max_iter, tolerance);
    //     let residual = compute_residual(matrix.local_nrow, &result, &exact);
    //     assert!(normr < tolerance);
    //     assert!(iterations < max_iter);
    //     assert!(residual < 1e-15);
    //     for (actual, expected) in result.iter().zip(exact) {
    //         assert!((expected - actual).abs() < 1e-5);
    //     }
    // }
}
