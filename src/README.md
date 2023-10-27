# Translating HPCCG C++ into Rust

## Files to translate

- Simple (~3hrs including repo setup)
  - [x] `ddot.cpp`
  - [x] `compute_residual.cpp`
  - [x] `waxpby.cpp`
- Data structure
  - [ ] `HPC_Sparse_Matrix.cpp`
  - [ ] `generate_matrix.cpp`
- Data structure dependents
  - [ ] `HPC_sparsemv.cpp`
  - [ ] `read_HPC_row.cpp`
- Driver code
  - [ ] `HPCCG.cpp`
  - [ ] `main.cpp`
  - [ ] `mytimer.cpp`
- Output format
  - [ ] `Yaml_Doc.cpp`
  - [ ] `Yaml_Element.cpp`
- Excluded
  - [x] `dump_matlab_matrix.cpp` (Unused data format)
  - [x] `exchange_externals.cpp` (Only required for MPI)
  - [x] `make_local_matrix.cpp` (Only required for MPI)

## Output goals

For each file:

- Direct translation
- Idiomatic translation
- Unit tests in Rust for translated Rust code