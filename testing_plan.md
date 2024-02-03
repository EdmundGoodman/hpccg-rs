
# Testing plan

Tool with YAML input to run and analyses tests

- best practices for YAML schema?
- what test harnesses exist already?

matrix run can want different data in diff columns (e.g. dimensions x memory or dimensions + memory x time)

groups of settings and generators for settings

directory structure of outputs named meaningfully and following YAML structure

## Input data / machine & build config

- Groups
  - Strong scaling
  - Weak scaling
  - Memory characterisation
    - In cache
    - In memory
    - Out of memory
  - Arbitrary input data generator/DSL?

## Versions

### Reference

- Serial
- OpenMP only
- MPI only
- OpenMP + MPI
- Roofline curve?

### Translated

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

## Build configurations/compile flags (optional?)

## Repeats/statistical methods
