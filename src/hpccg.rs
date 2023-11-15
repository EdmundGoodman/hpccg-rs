use crate::ddot::ddot_idiomatic;
use crate::mytimer::mytimer;
use crate::sparsemv::sparsemv_idiomatic_vec;
use crate::waxpby::waxpby_idiomatic;
use super::hpc_sparse_matrix::IdiomaticHpcSparseMatrix;

fn tick(t0: &mut f64) {
    *t0 = mytimer();
}

fn tock(t0: &f64, t: &mut f64) {
    *t += mytimer() - t0;
}

pub fn hpccg_direct(
    matrix: &IdiomaticHpcSparseMatrix,
    b: &Vec<f64>,
    x: &mut Vec<f64>,
    max_iter: i32,
    tolerance: f64,
    niters: &mut i32,
    normr: &mut f64,
    times: &mut Vec<f64>,
) {
    let t_begin: f64 = mytimer();

    let mut t0: f64 = 0.0;
    let mut t1: f64 = 0.0;
    let mut t2: f64 = 0.0;
    let mut t3: f64 = 0.0;
    let mut t4: f64 = 0.0;  // t4 only in MPI mode

    let nrow = matrix.local_nrow;
    let ncol = matrix.local_ncol;

    let mut r: Vec<f64> = Vec::with_capacity(nrow as usize);
    let mut p: Vec<f64> = Vec::with_capacity(ncol as usize);
    let mut Ap: Vec<f64> = Vec::with_capacity(nrow as usize);

    *normr = 0.0;
    let mut rtrans: f64 = 0.0;
    let mut oldrtrans: f64 = 0.0;

    let rank: i32 = 0;

    // let print_freq = (max_iter/10).max(1).min(50);
    let mut print_freq = max_iter/10;
    if print_freq > 50 {
        print_freq = 50;
    } else if print_freq < 1 {
        print_freq = 1;
    }

    // p is of length ncols, copy x to p for sparse MV operation
    tick(&mut t0);  //tick!(t0);
    p = waxpby_idiomatic(1.0, x, 0.0, b);
    tock(&t0, &mut t2);  //tock!(t0, t2);

    tick(&mut t0);
    Ap = sparsemv_idiomatic_vec(&matrix, &p);
    tock(&t0, &mut t3);

    tick(&mut t0);
    r = waxpby_idiomatic(1.0, b, -1.0, &Ap);
    tock(&t0, &mut t2);

    tick(&mut t0);
    rtrans = ddot_idiomatic(&r, &r);
    tock(&t0, &mut t1);

    *normr = rtrans.sqrt();

    println!("Initial Residual = {normr:+.5e}");

    for k in 1..max_iter {
        if *normr <= tolerance {
            break;
        }

        if k == 1 {
            tick(&mut t0);
            p = waxpby_idiomatic(1.0, &r, 0.0, &r);
            tock(&t0, &mut t2);
        } else {
            oldrtrans = rtrans;
            tick(&mut t0);
            rtrans = ddot_idiomatic(&r,&r);
            tock(&t0, &mut t1);
            let beta = rtrans/oldrtrans;
            tick(&mut t0);
            p = waxpby_idiomatic(1.0, &r, beta, &p);
            tock(&t0, &mut t2);
        }

        *normr = rtrans.sqrt();
        if k%print_freq == 0 || k+1 == max_iter {
            println!("Iteration = {k} , Residual = {normr:+.5e}");
        }

        tick(&mut t0);
        Ap = sparsemv_idiomatic_vec(matrix, &p);
        tock(&t0, &mut t3);

        tick(&mut t0);
        let alpha = ddot_idiomatic(&p, &Ap);
        tock(&t0, &mut t1);

        let alpha = rtrans/alpha;
        tick(&mut t0);
        *x = waxpby_idiomatic(1.0, x, alpha, &p);
        r = waxpby_idiomatic(1.0, &r, -alpha, &Ap);
        tock(&t0, &mut t2);
        *niters = k;
    }

    *times = vec![
        mytimer() - t_begin,
        t1, t2, t3, t4
    ];

}
