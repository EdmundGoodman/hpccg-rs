use mpi::traits::*;

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
/// * `row_start_inds` - A vector of pointers to values
/// * `list_of_vals` - A vector of values stored in the matrix
/// * `list_of_inds` - A vector of indices into the matrix
#[derive(Debug)]
#[allow(dead_code)]
pub struct SparseMatrix {
    pub start_row: usize,
    pub stop_row: usize,
    pub total_nrow: usize,
    pub total_nnz: usize,
    pub local_nrow: usize,
    pub local_ncol: usize,
    pub local_nnz: usize,
    pub nnz_in_row: Vec<usize>,
    pub row_start_inds: Vec<usize>,
    pub list_of_vals: Vec<f64>,
    pub list_of_inds: Vec<i32>,
    // MPI only
    pub num_external: usize, // Option<usize>,
    pub num_send_neighbors: usize,
    pub external_index: Vec<usize>,
    pub external_local_index: Vec<i32>,
    pub total_to_be_sent: usize,
    pub elements_to_send: Vec<i32>,
    pub neighbors: Vec<usize>,
    pub recv_length: Vec<usize>,
    pub send_length: Vec<usize>,
    pub send_buffer: Vec<f64>,
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
        nx: usize,
        ny: usize,
        nz: usize,
        world: &impl Communicator,
    ) -> (Self, Vec<f64>, Vec<f64>, Vec<f64>) {
        let size = world.size() as usize;
        let rank = world.rank() as usize;

        let use_7pt_stencil = false;

        // The size of our sub-block (must be non-zero)
        let local_nrow = nx * ny * nz;
        assert!(local_nrow > 0);
        // The approximate number of non-zeros per row (excluding boundary nodes)
        let local_nnz = 27 * local_nrow;

        // Total number of grid points in mesh
        let total_nrow = local_nrow * size;
        // Approximately 27 nonzeros per row (except for boundary nodes)
        let total_nnz = 27 * total_nrow;

        // In non-mpi mode, the total row, column, and non-zero sizes are the same as the local ones
        // let (total_nnz, total_nrow, local_ncol) = (local_nnz, local_nrow, local_nrow);
        let local_ncol = local_nrow;

        // Each processor gets a section of a chimney stack domain
        let start_row = local_nrow * rank;
        let stop_row = start_row + local_nrow - 1;

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
        let mut list_of_inds: Vec<i32> = Vec::with_capacity(local_nnz);

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
                                    && (curcol >= 0 && curcol < (total_nrow as i32))
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
                                        list_of_inds.push(curcol);
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
            row_start_inds,
            list_of_vals,
            list_of_inds,
            // ===== MPI only ===== //
            num_external: 0,
            num_send_neighbors: 0,
            external_index: vec![],
            external_local_index: vec![],
            total_to_be_sent: 0,
            elements_to_send: vec![],
            neighbors: vec![],
            recv_length: vec![],
            send_length: vec![],
            send_buffer: vec![],
        };
        (matrix, guess, rhs, exact)
    }
}
