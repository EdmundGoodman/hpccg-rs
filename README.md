# hpccg-rs

A rust translation of ["High Performance Computing Conjugate Gradients: The original Mantevo miniapp"](https://github.com/Mantevo/HPCCG).

## Project structure

The top-level directories in the repository are different translated versions of the HPCCG algorithm.

- `0_cpp_versions/` contains the code taken directly from HPCCG, split across various directories for different types of parallelism and build tooling
- `1_naive/` contains a direct serial rust translation, using the interior mutability pattern of `Rc<RefCell<>>` to represent pointers
- `2_indexed/` modifies the previous translation's sparse matrix data structure to use indexing over pointers
- `3_single_indexed/` modifies the previous translation to re-use indexes to reduce memory bandwidth
- `4_no_bounds_check/` modifies the previous translation to `unsafe`-ly avoid bound checks on vector indexing operations
- `5_iterators/` modifies the previous translation to leverage iterators to minimise unsafe indexing operations
- `6_parallel/` applies the `rayon` crate to leverage multi-threading over the previous translation
- `7_mpi/` modifies `5_iterators/` to add the MPI optional functionality using the `rs-mpi` crate
- `8_hybrid/` combines the previous two translations to leverage both multi-threading and MPI

The `__misc/` directory contains other translations such as proof-of-concepts for the polyglotest equivalence checking
approach, and a trial of the `sprs` crate for sparse matrix representations, which were not included in the
performance analysis trials.

## Code provenance declaration

All code in the `0_cpp_versions/` directory is duplicated or modified from Michael Heroux's implementation
of HPCCG in C++ as part of the Mantevo Suite. Significant modifications to this codebase beyond Heroux's original
implementation include adding documentation in the form of doxygen comments, moving build tooling from Makefiles
to CMake, and writing a Kokkos implementation to replace OpenMP for shared memory parallelism.

All other code is translated from scratch solely by Edmund Goodman, as an aspect of a third year project at the
University of Warwick titled "Assessing the suitability of Rust for performant and productive implementations of HPC codebases''.
