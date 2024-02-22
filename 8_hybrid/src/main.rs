use mpi::collective::SystemOperation;
use mpi::traits::*;

pub mod hpccg;

mod tests;

/// The driver code for the calculating the conjugate gradient.
///
/// First,the progam generatess the matrix, right hand side vector,
/// exact solution vector, and an initial guess. Then, it calls the
/// HPCCG conjugate gradient solver on the matrix and associated data.
/// Finally, it print the result of the solver, and information about
/// the performance of the computation.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (nx, ny, nz) = match &args.to_owned()[..] {
        [_, x, y, z] => (
            x.parse::<usize>().expect("Failed to parse number!"),
            y.parse::<usize>().expect("Failed to parse number!"),
            z.parse::<usize>().expect("Failed to parse number!"),
        ),
        _ => (5, 5, 5),
    };

    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    let (mut matrix, guess, rhs, exact) = hpccg::SparseMatrix::generate_matrix(nx, ny, nz, &world);
    let max_iter = 150;
    let tolerance = 0.0;

    // TODO: Add timer for overhead making the matrix
    let t6 = hpccg::mytimer();
    hpccg::make_local_matrix(&mut matrix, &world);
    let t6 = hpccg::mytimer() - t6;

    let (result, iterations, normr, mut times) =
        hpccg::solver(&mut matrix, &rhs, &guess, max_iter, tolerance, &world);

    let ddot_flops = iterations as i64 * 4 * matrix.total_nrow as i64;
    let waxpby_flops = iterations as i64 * 6 * matrix.total_nrow as i64;
    let sparsemv_flops = iterations as i64 * 2 * matrix.total_nnz as i64;
    let total_flops = ddot_flops + waxpby_flops + sparsemv_flops;

    times.push(t6);
    let total_sparsemv_time = times[3] + times[5] + times[6];
    let mut t4min = 0.0;
    let mut t4max = 0.0;
    let mut t4avg = 0.0;
    world.all_reduce_into(&times[4], &mut t4min, SystemOperation::min());
    world.all_reduce_into(&times[4], &mut t4max, SystemOperation::max());
    world.all_reduce_into(&times[4], &mut t4avg, SystemOperation::sum());

    if world.rank() == 0 {
        let residual = hpccg::compute_residual(matrix.local_nrow, &result, &exact);

        println!("Mini-Application Name: hpccg");
        println!("Mini-Application Version: 1.0");
        println!("Parallelism:");
        println!("  Number of MPI ranks: {}", world.size());
        println!("  Rayon disabled");
        println!("Dimensions:\n  nx: {nx}\n  ny: {ny}\n  nz: {nz}");
        println!("Number of iterations: {iterations}");
        println!("Final residual: {normr:.5e}");
        println!("#********** Performance Summary (times in sec) ***********");
        println!("Time Summary:");
        println!("  Total: {:.4}", times[0]);
        println!("  DDOT: {:.4}", times[1]);
        println!("  WAXPBY: {:.4}", times[2]);
        println!("  SPARSEMV: {:.4}", times[3]);
        println!("FLOPS Summary:");
        println!("  Total: {total_flops:.4}");
        println!("  DDOT: {ddot_flops:.4}");
        println!("  WAXPBY: {waxpby_flops:.4}");
        println!("  SPARSEMV: {sparsemv_flops:.4}");
        println!("MFLOPS Summary:");
        println!("  Total: {:.4}", (total_flops as f64) / times[0] / 1.0e6);
        println!("  DDOT: {:.4}", (ddot_flops as f64) / times[1] / 1.0e6);
        println!("  WAXPBY: {:.4}", (waxpby_flops as f64) / times[2] / 1.0e6);
        println!(
            "  SPARSEMV: {:.4}",
            (sparsemv_flops as f64) / times[3] / 1.0e6
        );
        println!("DDOT Timing Variations:");
        println!("  Min DDOT MPI_Allreduce time: {t4min:.4}");
        println!("  Max DDOT MPI_Allreduce time: {t4max:.4}");
        println!("  Avg DDOT MPI_Allreduce time: {t4avg:.4}");
        println!("SPARSEMV OVERHEADS:");
        println!(
            "  SPARSEMV MFLOPS W OVERHEAD: {:.4}",
            (sparsemv_flops as f64) / total_sparsemv_time / 1.0e6
        );
        println!(
            "  SPARSEMV PARALLEL OVERHEAD Time: {:.4}",
            times[5] + times[6]
        );
        println!(
            "  SPARSEMV PARALLEL OVERHEAD Pct: {:.4}",
            ((times[5] + times[6]) / total_sparsemv_time) * 100.0
        );
        println!("  SPARSEMV PARALLEL OVERHEAD Setup Time: {:.4}", times[6]);
        println!(
            "  SPARSEMV PARALLEL OVERHEAD Setup Pct: {:.4}",
            (times[6] / total_sparsemv_time) * 100.0
        );
        println!(
            "  SPARSEMV PARALLEL OVERHEAD Bdry Exch Time: {:.4}",
            times[5]
        );
        println!(
            "  SPARSEMV PARALLEL OVERHEAD Bdry Exch Pct: {:.4}",
            (times[5] / total_sparsemv_time) * 100.0
        );
        println!("Difference between computed and exact = {residual:.5e}.");
    }
}