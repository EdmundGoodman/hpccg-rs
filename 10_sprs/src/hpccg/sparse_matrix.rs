#[allow(unused_imports)]
use ndarray::{array, Array1};
use sprs::CsMat;

/// Generates the initial mesh and its associated values.
///
/// # Arguments
///  * `nx` - Size of x dimension.
///  * `ny` - Size of y dimension.
///  * `nz` - Size of z dimension.
///
/// # Return values
///  * `matrix` - Generated sparse matrix.
///  * `guess` - Inital guess for the mesh.
///  * `rhs` - Right hand side.
///  * `exact` - Exact solution (as computed by a direct solver).
pub fn generate_matrix(
    nx: usize,
    ny: usize,
    nz: usize,
) -> (CsMat<f64>, Array1<f64>, Array1<f64>, Array1<f64>) {
    let use_7pt_stencil = false;

    // The size of our sub-block (must be non-zero)
    let local_nrow = nx * ny * nz;
    assert!(local_nrow > 0);
    // The approximate number of non-zeros per row (excluding boundary nodes)
    let local_nnz = 27 * local_nrow;
    // Each processor gets a section of a chimney stack domain
    let start_row = 0;

    // In non-mpi mode, matrix is square
    let local_ncol = local_nrow;

    // The number of non-zero numbers in each row
    let mut nnz_in_row = Vec::with_capacity(local_nrow);
    // The index of the start of each row into `list_of_vals` and `list_of_inds`
    let mut row_start_inds: Vec<usize> = Vec::with_capacity(local_nrow);

    // Output data other than the sparse matrix
    let mut guess: Vec<f64> = Vec::with_capacity(local_nrow);
    let mut rhs: Vec<f64> = Vec::with_capacity(local_nrow);
    let mut exact: Vec<f64> = Vec::with_capacity(local_nrow);

    // Allocate arrays that are of length local_nnz
    let mut list_of_vals: Vec<f64> = Vec::with_capacity(local_nnz);
    let mut list_of_inds: Vec<usize> = Vec::with_capacity(local_nnz);

    let mut curvalind: usize = 0;
    for iz in 0..nz {
        for iy in 0..ny {
            for ix in 0..nx {
                let currow = start_row + iz * nx * ny + iy * nx + ix;
                let mut nnzrow: usize = 0;
                row_start_inds.push(curvalind);
                for sz in -1..=1 {
                    for sy in -1..=1 {
                        for sx in -1..=1 {
                            let curcol = (currow as i32)
                                + sz * (nx as i32) * (ny as i32)
                                + sy * (nx as i32)
                                + sx;
                            // Since we have a stack of nx by ny by nz domains , stacking
                            // in the z direction, we check to see if sx and sy are
                            // reaching outside of the domain, while the check for the
                            // curcol being valid is sufficient to check the z values
                            let sx_ix = (ix as i32) + sx;
                            let sy_iy = (iy as i32) + sy;
                            #[allow(clippy::collapsible_if)]
                            if (sx_ix >= 0)
                                && (sx_ix < (nx as i32))
                                && (sy_iy >= 0)
                                && (sy_iy < (ny as i32))
                                && (curcol >= 0 && curcol < (local_nrow as i32))
                            {
                                if !use_7pt_stencil || (sz * sz + sy * sy + sx * sx <= 1) {
                                    // This logic will skip over point that are not part of
                                    // a 7-pt stencil
                                    if (curcol as usize) == currow {
                                        list_of_vals.push(27.0);
                                    } else {
                                        list_of_vals.push(-1.0);
                                    }
                                    curvalind += 1;
                                    list_of_inds.push(curcol as usize);
                                    nnzrow += 1;
                                }
                            }
                        }
                    }
                }
                nnz_in_row.push(nnzrow);
                guess.push(0.0);
                rhs.push(27.0 - ((nnzrow - 1) as f64));
                exact.push(1.0);
            }
        }
    }

    let mut ind_ptrs = row_start_inds.clone();
    ind_ptrs.push(*ind_ptrs.last().unwrap() + nnz_in_row.last().unwrap());
    let matrix = CsMat::new(
        (local_nrow, local_ncol),
        ind_ptrs,
        list_of_inds,
        list_of_vals,
    );

    (
        matrix,
        Array1::from(guess),
        Array1::from(rhs),
        Array1::from(exact),
    )
}

#[test]
fn test_sparse_matrix() {
    let (matrix, guess, rhs, exact) = generate_matrix(2, 2, 2);
    assert_eq!(matrix.rows(), 8);
    assert_eq!(matrix.nnz(), 64);

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
    assert_eq!(matrix.data(), expected_vals);
    assert_eq!(matrix.indices(), expected_inds);

    assert_eq!(guess, array![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    assert_eq!(rhs, array![20.0, 20.0, 20.0, 20.0, 20.0, 20.0, 20.0, 20.0]);
    assert_eq!(exact, array![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
}
