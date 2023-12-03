use std::rc::Rc;
use std::cell::RefCell;
use crate::sparsemv::sparsemv;

/// A re-worked data structure representing a sparse matrix mesh
///
/// Instead of `nnz_in_row`,`ptr_to_vals_in_row`,...,`list_of_inds` we have a single
/// non-homogenous vector of rows, which are themselves vectors of pairs of indices
/// and their values
#[derive(Debug)]
#[allow(dead_code)]
pub struct HpcSparseMatrix {
    pub start_row: i32,
    pub stop_row: i32,
    pub total_nrow: i32,
    pub total_nnz: i32,
    pub local_nrow: i32,
    pub local_ncol: i32,
    pub local_nnz: i32,
    pub data: Vec<Vec<(f64, usize)>>
}

impl HpcSparseMatrix {
    /// Generates the initial mesh and its associated values
    ///
    /// # Arguments
    ///  * `nx` - Size of x dimension
    ///  * `ny` - Size of y dimension
    ///  * `nz` - Size of z dimension
    ///
    /// # Return values
    ///  * `matrix` - Generated sparse matrix
    ///  * `guess` - Inital guess for the mesh
    ///  * `rhs` - Right hand side
    ///  * `exact` - Exact solution (as computed by a direct solver)
    pub fn generate_matrix(nx: i32, ny: i32, nz: i32) -> (HpcSparseMatrix, Vec<f64>, Vec<f64>, Vec<f64>) {
        let (direct_matrix, x, b, xexact) = HpcSparseMatrixDirect::generate_matrix(nx, ny, nz);
        let matrix = HpcSparseMatrix::from(direct_matrix);
        (matrix, x, b, xexact)
    }
}

impl From<HpcSparseMatrixDirect> for HpcSparseMatrix {
    fn from(item: HpcSparseMatrixDirect) -> Self {
        let mut data = vec![];

        let nrows = item.local_nrow as usize;
        let mut row_start = 0;

        for row_num in 0..nrows {
            let mut row = vec![];
            let cur_nnz = item.nnz_in_row[row_num] as usize;
            for offset in 0..cur_nnz {
                // De-reference the value/index at the current position in the
                // contiguous list of values
                let val = *item.list_of_vals[row_start + offset].borrow();
                let ind = *item.list_of_inds[row_start + offset].borrow();
                row.push((val, ind as usize));
            }
            row_start += cur_nnz;
            data.push(row);
        }

        HpcSparseMatrix {
            start_row: item.start_row,
            stop_row: item.stop_row,
            total_nrow: item.total_nrow,
            total_nnz: item.total_nnz,
            local_nrow: item.local_nrow,
            local_ncol: item.local_ncol,
            local_nnz: item.local_nnz,
            data
        }
    }
}



#[derive(Debug)]
#[allow(dead_code)]
pub struct HpcSparseMatrixDirect {
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

impl HpcSparseMatrixDirect {
    pub fn generate_matrix(nx: i32, ny: i32, nz: i32) -> (HpcSparseMatrixDirect, Vec<f64>, Vec<f64>, Vec<f64>) {
        let use_7pt_stencil = false;

        // The size of our sub-block (must be non-zero)
        let local_nrow = nx * ny * nz;
        assert!(local_nrow > 0);
        // The approximate number of non-zeros per row (excluding boundary nodes)
        let local_nnz = 27*local_nrow;
        // Each processor gets a section of a chimney stack domain
        let start_row = 0;
        let stop_row = local_nrow - 1;


        // The number of non-zero numbers in each row
        let mut nnz_in_row = Vec::with_capacity(local_nrow as usize);
        // Arrays of reference-counted pointers
        let mut ptr_to_vals_in_row: Vec<Rc<RefCell<f64>>> = Vec::with_capacity(local_nrow as usize);
        let mut ptr_to_inds_in_row: Vec<Rc<RefCell<i32>>> = Vec::with_capacity(local_nrow as usize);
        let mut ptr_to_diags: Vec<Rc<RefCell<f64>>> = Vec::with_capacity(local_nrow as usize);

        // Output data other than the sparse matrix
        let mut guess : Vec<f64> = Vec::with_capacity(local_nrow as usize);
        let mut rhs : Vec<f64> = Vec::with_capacity(local_nrow as usize);
        let mut exact: Vec<f64> = Vec::with_capacity(local_nrow as usize);

        // Allocate arrays that are of length local_nnz
        // Either, we do reference counting, or we make the data structure less insane
        // let mut list_of_vals: Vec<Rc<RefCell<f64>>> = vec![Rc::new(RefCell::new(0.0)); local_nnz as usize]; //Vec::with_capacity(local_nnz as usize);
        let list_of_vals: Vec<Rc<RefCell<f64>>> = (0..local_nnz).map(|_| Rc::new(RefCell::new(0.0))).collect();
        // let mut list_of_inds: Vec<Rc<RefCell<i32>>> = vec![Rc::new(RefCell::new(0)); local_nnz as usize]; //Vec::with_capacity(local_nnz as usize);
        let list_of_inds: Vec<Rc<RefCell<i32>>> = (0..local_nnz).map(|_| Rc::new(RefCell::new(0))).collect();

        let mut curvalind: usize = 0;
        let mut curindind: usize = 0;

        for iz in 0..nz {
            for iy in 0..ny {
                for ix in 0..nx {

                    let currow = start_row+iz*nx*ny+iy*nx+ix;
                    let mut nnzrow = 0;
                    ptr_to_vals_in_row.push(Rc::clone(&list_of_vals[curvalind]));
                    ptr_to_inds_in_row.push(Rc::clone(&list_of_inds[curindind]));

                    for sz in -1..=1 {
                        for sy in -1..=1 {
                            for sx in -1..=1 {
                                let curcol = currow+sz*nx*ny+sy*nx+sx;
                                if (ix+sx>=0) && (ix+sx<nx) && (iy+sy>=0) && (iy+sy<ny) && (curcol>=0 && curcol<local_nrow) {
                                    // This logic will skip over point that are not part of a 7-pt stencil
                                    if !use_7pt_stencil || (sz*sz+sy*sy+sx*sx<=1) {
                                        if curcol==currow {
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
                    rhs.push(27.0 - ((nnzrow-1) as f64));
                    exact.push(1.0);
                }
            }
        }

        let matrix = HpcSparseMatrixDirect {
            start_row,
            stop_row,
            local_nrow,
            total_nnz: local_nnz,
            total_nrow: local_nrow,
            local_ncol: local_nrow,
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

    pub fn sparsemv_direct(&self, x: &Vec<f64>) -> Vec<f64> {
        let nrow = self.local_nrow as usize;

        let mut y = vec![0.0; x.len()];
        let mut start_index = 0;
        for i in 0..nrow {
            let mut sum = 0.0;
            // cur_vals is a vector slice with ptr_to_vals_in_row of length cur_nnz,
            // representing the values in the row
            // let cur_vals = matrix.ptr_to_vals_in_row[i];
            // let cur_inds = matrix.ptr_to_vals_in_row[i];
            let cur_nnz = self.nnz_in_row[i] as usize;

            for j in 0..cur_nnz {
                let val = *self.list_of_vals[start_index+j].borrow();
                let ind = *self.list_of_inds[start_index+j].borrow() as usize;
                sum += val * x[ind];
            }
            y[i] = sum;
            start_index += cur_nnz;
        }
        y
    }
}


#[test]
fn test_idiomatic_sparse_matrix() {
    let (matrix, _, _, _) = HpcSparseMatrixDirect::generate_matrix(2, 2, 2);
    let guess = vec![2.3; 8];

    let old = matrix.sparsemv_direct(&guess);
    let new_matrix = HpcSparseMatrix::from(matrix);
    let new = sparsemv(&new_matrix, &guess);

    assert_eq!(old, new);
}


#[test]
fn test_sparse_matrix() {
    let (matrix, guess, rhs, exact) = HpcSparseMatrixDirect::generate_matrix(2, 2, 2);

    assert_eq!(matrix.local_nrow, 8);
    assert_eq!(matrix.local_nnz, 216);
    assert_eq!(matrix.nnz_in_row, vec![8; 8]);

    let vals_in_row: Vec<f64> = matrix.ptr_to_vals_in_row.iter().map(|x| *x.borrow()).collect();
    let inds_in_row: Vec<i32> = matrix.ptr_to_inds_in_row.iter().map(|x| *x.borrow()).collect();
    let diags: Vec<f64> = matrix.ptr_to_diags.iter().map(|x| *x.borrow()).collect();
    assert_eq!(vals_in_row, vec![27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0]);
    assert_eq!(inds_in_row, vec![0; 8]);
    assert_eq!(diags, vec![27.0; 8]);

    let vals: Vec<f64> = matrix.list_of_vals.iter().map(|x| *x.borrow()).collect();
    let expected_vals =
        vec![27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 27.0, -1.0, -1.0, -1.0, -1.0,
             -1.0, -1.0, -1.0, -1.0, 27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 27.0,
             -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 27.0, -1.0, -1.0, -1.0, -1.0, -1.0,
             -1.0, -1.0, -1.0, 27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 27.0, -1.0,
             -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 27.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
             0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
             0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
             0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
             0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
             0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
             0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
             0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
             0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
             0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let inds: Vec<i32> = matrix.list_of_inds.iter().map(|x| *x.borrow()).collect();
    let expected_inds =
        vec![0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4,
             5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1,
             2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(vals, expected_vals);
    assert_eq!(inds, expected_inds);

    assert_eq!(guess, vec![0.0; 8]);
    assert_eq!(rhs, vec![20.0; 8]);
    assert_eq!(exact, vec![1.0; 8]);
}