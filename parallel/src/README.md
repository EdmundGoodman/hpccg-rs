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
  - [x] `HPCCG.cpp`
  - [x] `main.cpp`

- Excluded
  - [x] `dump_matlab_matrix.cpp` (Unused data format - could be useful for understanding?)
  - [x] `exchange_externals.cpp` (Only required for MPI)
  - [x] `make_local_matrix.cpp` (Only required for MPI)
  - [x] `read_HPC_row.cpp` (Only required for pre-generated matrices)
  - [x] `Yaml_Doc.cpp` (Only needed to dump data as YAML)
  - [x] `Yaml_Element.cpp` (Only needed to dump data as YAML)

## Output goals

### For each translated rust file

- Direct translation
- Idiomatic translation
- Unit tests in Rust for translated Rust code

### For the input C++ files

- Write documentation
- Give variables less monosyllabic names (shorter variable names don't make your code go faster!!!)
- Could PR or make more sane public fork explaining what things actually do?

## Notes


### Matrix generation

- Data structure is very dependent on interlinked arrays of pointers - could this be changed?
- `nnzglobal` is a completely unused variable!
- Consider re-writing c++ version to keep track of indices not pointers where possible?