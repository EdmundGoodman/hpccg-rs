
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

- [x] rsmpi on DCS
  - [x] works, reply to tech team
  - [x] add timing code
- [ ] rayon+mpi implementation
- [ ] roofline curves and stream benchmark
- [ ] benchmarking on DCS
  - [ ] update scripts to do all combinations, incl. compiler backends etc.
- [ ] benchmarking on SCRTP
  - [ ] get rustup, then run same script as dcs
- [ ] start kokkos implementation
- [ ] read perf book
- [ ] Optional: raw ffi:mpi implementation?
- [ ] Optional: iterators aren't zero cost abstraction
- [ ] Optional: work out what normr rtrans etc mean
- [ ] GCC vs clang comparison? Could impact iterators

## Testing plan

Tool with YAML input to run and analyses tests

- best practices for YAML schema?
- what test harnesses exist already?

matrix run can want different data in diff columns (e.g. dimensions x memory or dimensions + memory x time)

groups of settings and generators for settings

directory structure of outputs named meaningfully and following YAML structure

### Input data / machine & build config

- Groups
  - Strong scaling
  - Weak scaling
  - Memory characterisation
    - In cache
    - In memory
    - Out of memory
  - Arbitrary input data generator/DSL?

  - Cubic increasing, can extrapolate time for non-MPI versions
  - MPI/rayon total number of parallel equal, as product of parallel threads * nodes
  - Number of MPI nodes going up with powers of 2

### Versions

#### Reference

- Serial
- OpenMP only
- MPI only
- OpenMP + MPI
- Roofline curve?

#### Translated

- Naive
- Indexed
- Single_indexed
- No_bounds_check
- Iterators
- Parallel
- MPI
- MPI + Parallel

(optional/todo)

- (+ read perf book and get better version?)
- sprs library
- Kokkos

### Build configurations/compile flags (optional?)

### Repeats/statistical methods

- 5 drop top and bottom?