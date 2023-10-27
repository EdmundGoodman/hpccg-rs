#[derive(Debug)]
struct HpcSparseMatrix<'a> {
    title: &'a str,
    start_row: i32,
    stop_row: i32,
    total_nrow: i32,
    total_nnz: i64,
    local_nrow: i32,
    local_ncol: i32,
    local_nnz: i32,
    nnz_in_row: Vec<i64>,
    ptr_to_vals_in_row: Vec<&'a f64>,
    ptr_to_inds_in_row: Vec<&'a i32>,
    ptr_to_diags: Vec<&'a f64>,
    // Needed for cleaning up memory (in C++)
    list_of_vals: Vec<f64>,
    list_of_inds: Vec<i32>,
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
    // TODO: Consider changing from i32 to u8?
    fn generate_matrix(nx: i32, ny: i32, nz: i32) {
        // Define MPI settings for serial case
        let size = 1;
        let rank = 0;
        let use_7pt_stencil = false;

        // Initialise the struct
        // In Rust, all the values have to populate at initialise-time
        // let A = ...

        let local_nrow = nx * ny * nz;  // The size of our sub-block (must be non-zero)
        assert!(local_nrow > 0);
        let local_nnz = 27*local_nrow;  // The approximate number of non-zeros per row (excluding boundary nodes)

        let total_nrow = local_nrow*size;  // Total number of grid points in mesh
        let total_nnz = 27 * (total_nrow as i64);  // The approximate number of non-zeros per row (excluding boundary nodes)

        // Each processor gets a section of a chimney stack domain
        let start_row = local_nrow * rank;
        let stop_row = start_row + local_nrow -1 ;

        // TODO: Could box these onto the heap?
        let mut nnz_in_row = vec![0; local_nrow as usize];
        // Might need to be Vec<Option<&f64>> = vec![None; local_nrow as usize]
        let mut ptr_to_vals_in_row: Vec<&f64> = Vec::with_capacity(local_nrow as usize);
        let mut ptr_to_inds_in_row : Vec<&i32> = Vec::with_capacity(local_nrow as usize);
        let mut ptr_to_diags: Vec<&f64> = Vec::with_capacity(local_nrow as usize);

        // Output data other than the sparse matrix
        let mut x : Vec<f64> = Vec::with_capacity(local_nrow as usize);
        let mut b : Vec<f64> = Vec::with_capacity(local_nrow as usize);
        let mut xexact: Vec<f64> = Vec::with_capacity(local_nrow as usize);

        // Allocate arrays that are of length local_nnz
        // Either, we do reference counting, or we make the data structure less insane
        let mut list_of_vals = vec![0.0; local_nnz as usize];
        let mut list_of_inds = vec![0; local_nnz as usize];

        let mut curvalind: usize = 0;
        // let curvalptr: &f64 = &list_of_vals[curvalind];
        let mut curindind: usize = 0;
        // let curindptr: &i32 = &list_of_inds[curindind];

        let mut nnzglobal: i64 = 0;
        for iz in 0..nz {
            for iy in 0..ny {
                for ix in 0..nx {

                    let curlocalrow = (iz*nx*ny+iy*nx+ix) as usize;
                    let currow = start_row+iz*nx*ny+iy*nx+ix;
                    let mut nnzrow = 0;
                    ptr_to_vals_in_row[curlocalrow] = &list_of_vals[curvalind];
                    ptr_to_inds_in_row[curlocalrow] = &list_of_inds[curindind];

                    for sz in -1..=1 {
                        for sy in -1..=1 {
                            for sx in -1..=1 {
                                let curcol = currow+sz*nx*ny+sy*nx+sx;
                                if (ix+sx>=0) && (ix+sx<nx) && (iy+sy>=0) && (iy+sy<ny) && (curcol>=0 && curcol<total_nrow) {
                                    // This logic will skip over point that are not part of a 7-pt stencil
                                    if !use_7pt_stencil || (sz*sz+sy*sy+sx*sx<=1) {
                                        if curcol==currow {
                                            ptr_to_diags[curlocalrow] = &list_of_vals[curvalind];
                                            // *curvalptr++ = 27.0;
                                            list_of_vals[curvalind] = 27.0;
                                            curvalind += 1;
                                        } else {
                                            // *curvalptr++ = -1.0;
                                            list_of_vals[curvalind] = -1.0;
                                            curvalind += 1;
                                        }
                                        // *curindptr++ = curcol;
                                        list_of_inds[curindind] = curcol;
                                        curindind += 1;
                                        nnzrow += 1;
                                    }
                                }
                            }
                        }
                    }
                    nnz_in_row[curlocalrow] = nnzrow;
                    nnzglobal += nnzrow;
                    x[curlocalrow] = 0.0;
                    b[curlocalrow] = 27.0 - ((nnzrow-1) as f64);
                    xexact[curlocalrow] = 1.0;
                }
            }
        }

        let binding = String::from("Sparse matrix");
        let hpc_sparse_matrix = HpcSparseMatrix {
            title: binding.as_str(),
            start_row,
            stop_row,
            total_nrow,
            total_nnz,
            local_nrow,
            local_ncol: local_nrow,
            local_nnz,
            nnz_in_row,
            ptr_to_vals_in_row,
            ptr_to_inds_in_row,
            ptr_to_diags,
            list_of_vals: list_of_vals.clone(),
            list_of_inds: list_of_inds.clone(),
        };

        println!("{:?}", hpc_sparse_matrix)

    }
}


#[test]
fn test_ddot_idiomatic() {
    HpcSparseMatrix::generate_matrix(5, 5, 5);
}