
#pragma once

/**
 * A method to compute the dot product of two vectors.
 *
 * This function optimises caching by only accessing one of the vectors if both of the
 * input values point to the same vector.
 * `time_allreduce` is a pass-by-reference variable which is updated to track the total time
 * spent by MPI's reduce function whilst collecting partial sums.
 *
 * @param n The number of vector elements (on this processor).
 * @param x The first input vector.
 * @param y The second input vector.
 * @param result A pointer to a scalar value, which is updated to contain the calculated dot
 * product.
 * @param time_allreduce A mutable variable tracking the time spent by MPI's reduce function.
 * @return An exit code of zero on success.
 */
inline int ddot(const int n, const double* const x, const double* const y, double* const result) {
    double local_result = 0.0;
    if (y == x)
        for (int i = 0; i < n; i++) local_result += x[i] * x[i];
    else
        for (int i = 0; i < n; i++) local_result += x[i] * y[i];

    *result = local_result;
    return (0);
}
