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