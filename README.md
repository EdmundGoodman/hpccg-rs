# HPCCG-rs

A rust translation of "High Performance Computing Conjugate Gradients: The original Mantevo miniapp".

## Project structure

The top-level directories in the repository are different translated versions of the HPCCG algorithm.

- `original` contains the code taken directly from HPCCG
- `direct_serial` contains a direct serial rust translation, using the `Rc<RefCell<>>` construct to represent pointers
- `datastruct_serial` contains a serial rust translation, where the datastructure is modified for convenience
- `parallel` contains a parallelised rust translation, applying the `rayon` crate on top of `datastruct_serial`

## TODO

- Unit tests for original HPCCG code
- Modification to original HPCCG code to not be in CCS form
  - Write blog about sparse matrix structures, related to HPCCG
- Check raw generation to find memory not freed
- PR improved docs for HPCCG
- Think about what tooling flows would be helpful
  - Characterising/understanding existing large codebases
  - Unit testing for TDD
  - Equivalence checking methodology
- Check it iter is actually 0 cost abstraction by annotated code analysis (find the mov!)
- correctness by comparing perf characteristics
- macros for switching between rust and c++ unit test mode

converting pointer arithmetic to array indexing - does this incur a performance cost?

## New todo

- PR fix to UK-MAC website broken links
- HPCCG
  - Finish MPI translation
    - Clean up/TODOS
    - Documentation/fixes in original HPCCG code?
    - Consider additional using direct ffi::MPI ?
  - Fix and PR docs and CMakelist for original code
  - Investigate using sprs matrix library
    - Done, no sparse/dense parallelisation so slow
  - Investigate auto-vectorisation
  - Investigate zero-cost abstractions
  - Unit testing framework improve ergonomics
  - Improve testing script to include uncertainties/memory characterisation
  - Review good papers for more todos
  - (Kokkos version in C++)
  - (Investigate clustering techniques other than MPI)
- MiniMD
  - Translate single kernel, following workflow
- polyglotest
  - "a testing framework to empower pure TDD when translating rust to C++"
  - should be a crate
  - add a new cargo command `ffi_test` or similar
    - like cargo mpirun
    - runs unit tests on C++
  - cargo test runs as normal (or adds ffi_test alongside integration/unit/doc?)
  - Consider wrapper for autocxx for this purpose? macros in test?

## Things to do this week

- read perf book
- rsmpi on DCS
  - works, reply to tech team
  - add timing code
- benchmarking on DCS
  - update scripts to do all combinations, incl. compiler backends etc.
- benchmarking on SCRTP
  - get rustup, then run same script as dcs
- rayon+mpi implementation
- roofline curves and stream benchmark
- start kokkos implementation
- (raw ffi:mpi implementation?)
- (iterators aren't zero cost abstraction)

GCC vs clang comparison? Could impact iterators
