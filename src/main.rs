pub mod hpccg;

use hpccg::{solver, SparseMatrix};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (nx, ny, nz) = (
        args[1].parse::<i32>().expect("Failed to parse number!"),
        args[2].parse::<i32>().expect("Failed to parse number!"),
        args[3].parse::<i32>().expect("Failed to parse number!"),
    );
    println!("{nx} {ny} {nz}");

    let matrix = SparseMatrix { start_row: 2, stop_row: 2 };
    let result = solver(matrix);

    println!("{result}");
}
