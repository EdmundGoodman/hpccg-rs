pub mod sparse_matrix;
pub mod computer_residual;
mod ddot;
mod waxpby;
mod mytimer;
mod sparsmv;


pub use sparse_matrix::SparseMatrix;
use mytimer::mytimer;
use ddot::ddot;
use waxpby::waxpby;
use sparsmv::sparsemv;

/// Store the start time for a code section.
fn tick(t0: &mut f64) {
    *t0 = mytimer();
}

/// Increment the total time for a code section by the most recent interval.
fn tock(t0: &f64, t: &mut f64) {
    *t += mytimer() - t0;
}


/// A method to computer the approximate solution to `Ax = b`
///
/// # Arguments
/// * `matrix` - The input sparse matrix.
/// * `b` - The known right hand side vector.
/// * `x` - The current approximate solution, which starts as the initial guess.
/// * `max_iter` - The maximum number of iterations to perform.
/// * `tolerance` - The value the residual needs to be less than for convergence (how "good" of a
///                 solution do we need).
///
/// # Return values
/// * `result` - The approximate result at the end of the solver loop
/// * `iterations` - The number of iterations for which the solver ran
/// * `normr` - The residual difference between the current approximate solution and the exact
///             solution.
/// * `times` - An array of times spent for each operation (ddot/waxpby/sparse_mv/total).
pub fn solver(
    matrix: &SparseMatrix,
    rhs: &Vec<f64>,
    guess: &Vec<f64>,
    max_iterations: i32,
    tolerance: f64
) -> (Vec<f64>, i32, f64, Vec<f64>) {
    
    let t_begin: f64 = mytimer();
    let mut t0: f64 = 0.0;
    let mut t1: f64 = 0.0;
    let mut t2: f64 = 0.0;
    let mut t3: f64 = 0.0;
    // `t4` only used in MPI mode
    let t4: f64 = 0.0;

    let nrow = matrix.local_nrow as usize;
    let ncol = matrix.local_ncol as usize;
    // `rank` only used in MPI mode
    let rank: i32 = 0;

    let mut r: Vec<f64> = Vec::with_capacity(nrow as usize);
    let mut p: Vec<f64> = Vec::with_capacity(ncol as usize);
    let mut Ap: Vec<f64> = Vec::with_capacity(nrow as usize);

    let mut result = guess.clone();
    let mut iteration = 0;
    let mut normr = 0.0;
    let mut rtrans: f64 = 0.0;
    let mut oldrtrans: f64 = 0.0;

    // let print_freq = (max_iter/10).max(1).min(50);
    let mut print_freq = max_iterations /10;
    if print_freq > 50 {
        print_freq = 50;
    } else if print_freq < 1 {
        print_freq = 1;
    }


    // p is of length ncols, copy x to p for sparse MV operation
    tick(&mut t0);
    p = waxpby(result.len(), 1.0, &result, 0.0, rhs);
    tock(&t0, &mut t2);

    tick(&mut t0);
    Ap = sparsemv(&matrix, &p);
    tock(&t0, &mut t3);

    tick(&mut t0);
    r = waxpby(result.len(), 1.0, rhs, -1.0, &Ap);
    tock(&t0, &mut t2);

    tick(&mut t0);
    rtrans = ddot(r.len(), &r, &r);
    tock(&t0, &mut t1);

    normr = rtrans.sqrt();

    println!("Initial Residual = {normr:+.5e}");

    for k in 1..max_iterations {
        if normr <= tolerance {
            break;
        }

        if k == 1 {
            tick(&mut t0);
            p = waxpby(nrow, 1.0, &r, 0.0, &r);
            tock(&t0, &mut t2);
        } else {
            oldrtrans = rtrans;
            tick(&mut t0);
            rtrans = ddot(nrow,&r,&r);
            tock(&t0, &mut t1);
            let beta = rtrans/oldrtrans;
            tick(&mut t0);
            p = waxpby(nrow, 1.0, &r, beta, &p);
            tock(&t0, &mut t2);
        }

        normr = rtrans.sqrt();
        if k%print_freq == 0 || k+1 == max_iterations {
            println!("Iteration = {k} , Residual = {normr:+.5e}");
        }

        tick(&mut t0);
        Ap = sparsemv(matrix, &p);
        tock(&t0, &mut t3);

        tick(&mut t0);
        let alpha = ddot(r.len(),&p, &Ap);
        tock(&t0, &mut t1);

        let alpha = rtrans/alpha;
        tick(&mut t0);
        result = waxpby(nrow, 1.0, &result, alpha, &p);
        r = waxpby(nrow, 1.0, &r, -alpha, &Ap);
        tock(&t0, &mut t2);
        iteration = k;
    }

    let times = vec![
        mytimer() - t_begin,
        t1, t2, t3, t4
    ];

    (result, iteration, normr, times)
}
