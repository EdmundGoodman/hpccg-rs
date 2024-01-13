use mpi::traits::*;
use mpi::environment::Universe;
use once_cell::sync::Lazy;

pub static UNIVERSE: Lazy<Universe> = Lazy::new(||
    mpi::initialize().unwrap()
);