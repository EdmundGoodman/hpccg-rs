#[cfg(test)]
mod unit_tests {
    use mpi::environment::Universe;
    use once_cell::sync::Lazy;

    use crate::hpccg::hpccg_internals::{ddot, sparsemv, waxpby};
    use crate::hpccg::{compute_residual, solver, SparseMatrix};

    static UNIVERSE: Lazy<Universe> = Lazy::new(||
        mpi::initialize().unwrap()
    );

    #[test]
    fn test_compute_residual() {
        let width = 3;
        let vx = vec![1.0, 2.0, 3.0];
        let vy = vec![3.0, 2.0, 1.0];
        let r = compute_residual(width, &vx, &vy);
        assert_eq!(r, 2.0);
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


    #[test]
    fn test_sparse_matrix() {
        let (matrix, guess, rhs, exact) = SparseMatrix::generate_matrix(2, 2, 2, &UNIVERSE.world());
        assert_eq!(matrix.local_nrow, 8);
        assert_eq!(matrix.local_nnz, 216);
        assert_eq!(matrix.nnz_in_row, vec![8; 8]);

        let vals_in_row: Vec<f64> = matrix
            .row_start_inds
            .iter()
            .map(|&x| matrix.list_of_vals[x])
            .collect();
        let inds_in_row: Vec<i32> = matrix
            .row_start_inds
            .iter()
            .map(|&x| matrix.list_of_inds[x])
            .collect();
        assert_eq!(
            vals_in_row,
            vec![27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0]
        );
        assert_eq!(inds_in_row, vec![0; 8]);

        let expected_vals = vec![
            27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 27.0, -1.0, -1.0, -1.0, -1.0, -1.0,
            -1.0, -1.0, -1.0, 27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 27.0, -1.0, -1.0,
            -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
            27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 27.0, -1.0, -1.0, -1.0, -1.0, -1.0,
            -1.0, -1.0, -1.0, 27.0,
        ];
        let expected_inds = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5,
            6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3,
            4, 5, 6, 7,
        ];
        assert_eq!(matrix.list_of_vals, expected_vals);
        assert_eq!(matrix.list_of_inds, expected_inds);

        assert_eq!(guess, vec![0.0; 8]);
        assert_eq!(rhs, vec![20.0; 8]);
        assert_eq!(exact, vec![1.0; 8]);
    }

    #[test]
    fn test_sparsemv() {
        let (matrix, _, _, _) = SparseMatrix::generate_matrix(2, 2, 2, &UNIVERSE.world());
        let vx = vec![20.0; 8];
        let vy = sparsemv(&matrix, &vx);
        assert_eq!(vy, vec![400.0; 8]);

        let (matrix, _, _, _) = SparseMatrix::generate_matrix(3, 3, 3, &UNIVERSE.world());
        let vx = vec![
            20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 10.0, 1.0, 10.0,
            16.0, 10.0, 16.0, 20.0, 16.0, 20.0, 16.0, 10.0, 16.0, 20.0, 16.0, 20.0,
        ];
        let expected_vy = vec![
            461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 21.0,
            -385.0, 21.0, 287.0, 21.0, 287.0, 461.0, 287.0, 461.0, 287.0, 21.0, 287.0, 461.0, 287.0,
            461.0,
        ];
        let vy = sparsemv(&matrix, &vx);
        assert_eq!(vy, expected_vy);
    }


    #[test]
    fn test_waxpby() {
        let width = 3;
        let vx = vec![1.0, 2.0, 3.0];
        let vy = vec![3.0, 2.0, 1.0];
        let alpha = 4.0;
        let beta = 5.0;
        let result = waxpby(width, alpha, &vx, beta, &vy);
        assert_eq!(result, vec![19.0, 18.0, 17.0]);
        let alpha = 1.0;
        let result = waxpby(width, alpha, &vx, beta, &vy);
        assert_eq!(result, vec![16.0, 12.0, 8.0]);
        let alpha = 4.0;
        let beta = 1.0;
        let result = waxpby(width, alpha, &vx, beta, &vy);
        assert_eq!(result, vec![7.0, 10.0, 13.0]);
    }

    #[test]
    fn test_solver() {
        let (nx, ny, nz) = (5, 5, 5);
        let (matrix, guess, rhs, exact) = SparseMatrix::generate_matrix(nx, ny, nz, &UNIVERSE.world());
        let max_iter = 150;
        let tolerance = 5e-40;
        let (result, iterations, normr, _) = solver(&matrix, &rhs, &guess, max_iter, tolerance, &UNIVERSE.world());
        let residual = compute_residual(matrix.local_nrow, &result, &exact);
        assert!(normr < tolerance);
        assert!(iterations < max_iter);
        assert!(residual < 1e-15);
        for (actual, expected) in result.iter().zip(exact) {
            assert!((expected - actual).abs() < 1e-5);
        }
    }

}
