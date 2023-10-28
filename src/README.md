# Translating HPCCG C++ into Rust

## Files to translate

- Simple (~3hrs including repo setup)
  - [x] `ddot.cpp`
  - [x] `compute_residual.cpp`
  - [x] `waxpby.cpp`
- Data structure
  - [x] `HPC_Sparse_Matrix.cpp`
  - [x] `generate_matrix.cpp`
- Data structure dependents
  - [x] `HPC_sparsemv.cpp`
- Driver code
  - [x] `mytimer.cpp`
  - [ ] `HPCCG.cpp`
  - [ ] `main.cpp`

- Output format
  - [ ] `Yaml_Doc.cpp`
  - [ ] `Yaml_Element.cpp`
- Excluded
  - [x] `dump_matlab_matrix.cpp` (Unused data format)
  - [x] `exchange_externals.cpp` (Only required for MPI)
  - [x] `make_local_matrix.cpp` (Only required for MPI)
  - [ ] `read_HPC_row.cpp` (Only required for pre-generated matrices)

## Output goals

For each file:

- Direct translation
- Idiomatic translation
- Unit tests in Rust for translated Rust code


## Notes

### Matrix generation

- Data structure is very dependent on interlinked arrays of pointers - could this be changed?
- `nnzglobal` is a completely unused variable!
- Consider re-writing c++ version to keep track of indices not pointers where possible?