use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct HpcSparseMatrix<'a> {
    title: &'a str,
    start_row: i32,
    stop_row: i32,
    total_nrow: i32,
    total_nnz: i32,
    local_nrow: i32,
    local_ncol: i32,
    local_nnz: i32,
    nnz_in_row: Vec<i64>,
    ptr_to_vals_in_row: Vec<Rc<RefCell<f64>>>,
    ptr_to_inds_in_row: Vec<Rc<RefCell<i32>>>,
    ptr_to_diags: Vec<Rc<RefCell<f64>>>,
    // Needed for cleaning up memory (in C++)
    list_of_vals: Vec<Rc<RefCell<f64>>>,
    list_of_inds: Vec<Rc<RefCell<i32>>>,
}

/**
 * @brief Generates the inital mesh and values
 *
 * @param nx Size of x dimension
 * @param ny Size of y dimension
 * @param nz Size of z dimension
 * @param A Sparse matrix
 * @param x Inital guess for the mesh
 * @param b Right hand side
 * @param xexact Exact solution (as computed by a direct solver)
 * @param use_7pt_stencil true if using 7 point stencil, otherwise use 27 point stencil
 */
impl HpcSparseMatrix<'_> {

    /// Generates the inital mesh and values
    ///
    /// # Arguments
    ///  * `nx` - Size of x dimension
    ///  * `ny` - Size of y dimension
    ///  * `nz` - Size of z dimension
    ///
    /// # Return values
    ///  * `A` - Sparse matrix
    ///  * `x` - Inital guess for the mesh
    ///  * `b` - Right hand side
    ///  * `xexact` - Exact solution (as computed by a direct solver)
    fn generate_matrix(nx: i32, ny: i32, nz: i32) {
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
        let mut x : Vec<f64> = Vec::with_capacity(local_nrow as usize);
        let mut b : Vec<f64> = Vec::with_capacity(local_nrow as usize);
        let mut xexact: Vec<f64> = Vec::with_capacity(local_nrow as usize);

        // Allocate arrays that are of length local_nnz
        // Either, we do reference counting, or we make the data structure less insane
        // let mut list_of_vals: Vec<Rc<RefCell<f64>>> = vec![Rc::new(RefCell::new(0.0)); local_nnz as usize]; //Vec::with_capacity(local_nnz as usize);
        let mut list_of_vals: Vec<Rc<RefCell<f64>>> = (0..local_nnz).map(|_| Rc::new(RefCell::new(0.0))).collect();
        // let mut list_of_inds: Vec<Rc<RefCell<i32>>> = vec![Rc::new(RefCell::new(0)); local_nnz as usize]; //Vec::with_capacity(local_nnz as usize);
        let mut list_of_inds: Vec<Rc<RefCell<i32>>> = (0..local_nnz).map(|_| Rc::new(RefCell::new(0))).collect();

        let mut curvalind: usize = 0;
        let mut curindind: usize = 0;

        for iz in 0..nz {
            for iy in 0..ny {
                for ix in 0..nx {

                    let curlocalrow = (iz*nx*ny+iy*nx+ix) as usize;
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
                    x.push(0.0);
                    b.push(27.0 - ((nnzrow-1) as f64));
                    xexact.push(1.0);
                }
            }
        }

        println!("Matrix has {local_nrow} rows, with {local_nnz} non-zero values");

        println!("\n\n");
        println!("list_of_vals: {:?}", list_of_vals);
        println!("list_of_inds: {:?}", list_of_inds);
        // println!("list_of_inds item: {:?}", Rc::clone(&list_of_inds[2]));
        println!("\n");
        println!("nnz_in_row: {:?}", nnz_in_row);
        println!("ptr_to_vals_in_row: {:?}", ptr_to_vals_in_row);
        println!("ptr_to_inds_in_row: {:?}", ptr_to_inds_in_row);
        println!("ptr_to_diags: {:?}", ptr_to_diags);
        println!("\n\n");

        println!("x: {:?}", x);
        println!("b: {:?}", b);
        println!("xexact: {:?}", xexact);

        let binding = String::from("Sparse matrix");
        let A = HpcSparseMatrix {
            title: binding.as_str(),
            start_row,
            stop_row,
            local_nrow,
            total_nnz: local_nnz,
            total_nrow: local_nrow,
            local_ncol: local_nrow,
            local_nnz,
            nnz_in_row,
            ptr_to_vals_in_row: ptr_to_vals_in_row.clone(),
            ptr_to_inds_in_row: ptr_to_inds_in_row.clone(),
            ptr_to_diags: ptr_to_diags.clone(),
            list_of_vals: list_of_vals.clone(),
            list_of_inds: list_of_inds.clone(),
        };

        // println!("{:?}", A)

    }
}


#[test]
fn test_sparse_matrix() {
    HpcSparseMatrix::generate_matrix(5, 5, 5);
}