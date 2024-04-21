# Versions of the HPCCG mini-app with C++ preprocessor conditionals stripped

This allows fair comparison of productivity using the `scc` tool across version.

For example, to strip the serial only version, run the command:

```bash
unifdef 0_ref/* -m -UUSING_OMP -UUSING_MPI
```
