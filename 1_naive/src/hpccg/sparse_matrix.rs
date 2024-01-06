use std::cell::RefCell;
use std::rc::Rc;

/// A data structure representing a sparse matrix mesh
///
/// # Fields
/// * `start_row` - The row to start generating the matrix from (always `0` in serial mode)
/// * `stop_row` - The row to stop generating the matrix at (always `x*y*z-1` in serial mode)
/// * `total_nrow` - The total volume of the matrix (always equal to `local_nrow` in serial mode)
/// * `total_nnz` - The total number of non-zeroes (always equal to `local_nnz` in serial mode)
/// * `local_nrow` - The local volume of the matrix, calculated as `x*y*z` in serial mode
/// * `local_ncol` - A variable only used in MPI mode (set to `local_nrow` in serial mode)
/// * `local_nnz` - The local number of non-zero values, approximated as `local_nrow*27`
/// * `nnz_in_row` - A vector containing the number of non-zeroes in each row
/// * `ptr_to_vals_in_row` - A vector of pointers to values
/// * `ptr_to_inds_in_row` - A vector of pointers to indices
/// * `ptr_to_diags` -  A vector of pointers to diagonals
/// * `list_of_vals` - A vector of values stored in the matrix
/// * `list_of_inds` - A vector of indices into the matrix
#[derive(Debug)]
#[allow(dead_code)]
pub struct SparseMatrix {
    pub start_row: i32,
    pub stop_row: i32,
    pub total_nrow: i32,
    pub total_nnz: i32,
    pub local_nrow: i32,
    pub local_ncol: i32,
    pub local_nnz: i32,
    pub nnz_in_row: Vec<i64>,
    pub ptr_to_vals_in_row: Vec<Rc<RefCell<f64>>>,
    pub ptr_to_inds_in_row: Vec<Rc<RefCell<i32>>>,
    pub ptr_to_diags: Vec<Rc<RefCell<f64>>>,
    pub list_of_vals: Vec<Rc<RefCell<f64>>>,
    pub list_of_inds: Vec<Rc<RefCell<i32>>>,
}

impl SparseMatrix {
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
        nx: i32,
        ny: i32,
        nz: i32,
    ) -> (Self, Vec<f64>, Vec<f64>, Vec<f64>) {
        let use_7pt_stencil = false;

        // The size of our sub-block (must be non-zero)
        let local_nrow = nx * ny * nz;
        assert!(local_nrow > 0);
        // The approximate number of non-zeros per row (excluding boundary nodes)
        let local_nnz = 27 * local_nrow;
        // Each processor gets a section of a chimney stack domain
        let start_row = 0;
        let stop_row = local_nrow - 1;

        // In non-mpi mode, the total row, column, and non-zero sizes are the same as the local ones
        let (total_nnz, total_nrow, local_ncol) = (local_nnz, local_nrow, local_nrow);

        // The number of non-zero numbers in each row
        let mut nnz_in_row = Vec::with_capacity(local_nrow as usize);
        // Arrays of reference-counted pointers
        let mut ptr_to_vals_in_row: Vec<Rc<RefCell<f64>>> = Vec::with_capacity(local_nrow as usize);
        let mut ptr_to_inds_in_row: Vec<Rc<RefCell<i32>>> = Vec::with_capacity(local_nrow as usize);
        let mut ptr_to_diags: Vec<Rc<RefCell<f64>>> = Vec::with_capacity(local_nrow as usize);

        // Output data other than the sparse matrix
        let mut guess: Vec<f64> = Vec::with_capacity(local_nrow as usize);
        let mut rhs: Vec<f64> = Vec::with_capacity(local_nrow as usize);
        let mut exact: Vec<f64> = Vec::with_capacity(local_nrow as usize);

        // Allocate arrays that are of length local_nnz
        // Either, we do reference counting, or we make the data structure less insane
        // let mut list_of_vals: Vec<Rc<RefCell<f64>>> = vec![Rc::new(RefCell::new(0.0)); local_nnz as usize]; //Vec::with_capacity(local_nnz as usize);
        let list_of_vals: Vec<Rc<RefCell<f64>>> =
            (0..local_nnz).map(|_| Rc::new(RefCell::new(0.0))).collect();
        // let mut list_of_inds: Vec<Rc<RefCell<i32>>> = vec![Rc::new(RefCell::new(0)); local_nnz as usize]; //Vec::with_capacity(local_nnz as usize);
        let list_of_inds: Vec<Rc<RefCell<i32>>> =
            (0..local_nnz).map(|_| Rc::new(RefCell::new(0))).collect();

        let mut curvalind: usize = 0;
        let mut curindind: usize = 0;

        for iz in 0..nz {
            for iy in 0..ny {
                for ix in 0..nx {
                    let currow = start_row + iz * nx * ny + iy * nx + ix;
                    let mut nnzrow = 0;
                    ptr_to_vals_in_row.push(Rc::clone(&list_of_vals[curvalind]));
                    ptr_to_inds_in_row.push(Rc::clone(&list_of_inds[curindind]));

                    for sz in -1..=1 {
                        for sy in -1..=1 {
                            for sx in -1..=1 {
                                let curcol = currow + sz * nx * ny + sy * nx + sx;
                                // Since we have a stack of nx by ny by nz domains , stacking
                                // in the z direction, we check to see if sx and sy are
                                // reaching outside of the domain, while the check for the
                                // curcol being valid is sufficient to check the z values
                                #[allow(clippy::collapsible_if)]
                                if (ix + sx >= 0)
                                    && (ix + sx < nx)
                                    && (iy + sy >= 0)
                                    && (iy + sy < ny)
                                    && (curcol >= 0 && curcol < local_nrow)
                                {
                                    // This logic will skip over point that are not part of a 7-pt stencil
                                    if !use_7pt_stencil || (sz * sz + sy * sy + sx * sx <= 1) {
                                        if curcol == currow {
                                            ptr_to_diags.push(Rc::clone(&list_of_vals[curvalind]));
                                            // *curvalptr++ = 27.0; // post-increment
                                            *list_of_vals[curvalind].borrow_mut() = 27.0;
                                            curvalind += 1;
                                        } else {
                                            // *curvalptr++ = -1.0; // post-increment
                                            *list_of_vals[curvalind].borrow_mut() = -1.0;
                                            curvalind += 1;
                                        }
                                        // *curindptr++ = curcol; // post-increment
                                        *list_of_inds[curindind].borrow_mut() = curcol;
                                        curindind += 1;
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

        let matrix = SparseMatrix {
            start_row,
            stop_row,
            local_nrow,
            total_nnz,
            total_nrow,
            local_ncol,
            local_nnz,
            nnz_in_row,
            ptr_to_vals_in_row,
            ptr_to_inds_in_row,
            ptr_to_diags,
            list_of_vals,
            list_of_inds,
        };

        (matrix, guess, rhs, exact)
    }
}

#[test]
fn test_sparse_matrix() {
    let (matrix, guess, rhs, exact) = SparseMatrix::generate_matrix(2, 2, 2);

    assert_eq!(matrix.local_nrow, 8);
    assert_eq!(matrix.local_nnz, 216);
    assert_eq!(matrix.nnz_in_row, vec![8; 8]);

    let vals_in_row: Vec<f64> = matrix
        .ptr_to_vals_in_row
        .iter()
        .map(|x| *x.borrow())
        .collect();
    let inds_in_row: Vec<i32> = matrix
        .ptr_to_inds_in_row
        .iter()
        .map(|x| *x.borrow())
        .collect();
    let diags: Vec<f64> = matrix.ptr_to_diags.iter().map(|x| *x.borrow()).collect();
    assert_eq!(
        vals_in_row,
        vec![27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0]
    );
    assert_eq!(inds_in_row, vec![0; 8]);
    assert_eq!(diags, vec![27.0; 8]);

    let vals: Vec<f64> = matrix.list_of_vals.iter().map(|x| *x.borrow()).collect();
    let expected_vals = vec![
        27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 27.0, -1.0, -1.0, -1.0, -1.0, -1.0,
        -1.0, -1.0, -1.0, 27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 27.0, -1.0, -1.0,
        -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
        27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 27.0, -1.0, -1.0, -1.0, -1.0, -1.0,
        -1.0, -1.0, -1.0, 27.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];
    let inds: Vec<i32> = matrix.list_of_inds.iter().map(|x| *x.borrow()).collect();
    let expected_inds = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5,
        6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3,
        4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(vals, expected_vals);
    assert_eq!(inds, expected_inds);

    assert_eq!(guess, vec![0.0; 8]);
    assert_eq!(rhs, vec![20.0; 8]);
    assert_eq!(exact, vec![1.0; 8]);
}
