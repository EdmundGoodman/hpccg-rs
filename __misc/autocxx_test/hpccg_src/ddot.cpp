// ************************************************************************
//
//               HPCCG: Simple Conjugate Gradient Benchmark Code
//                 Copyright (2006) Sandia Corporation
//
// Under terms of Contract DE-AC04-94AL85000, there is a non-exclusive
// license for use of this work by or on behalf of the U.S. Government.
//
// BSD 3-Clause License
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// * Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.
//
// * Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.
//
// * Neither the name of the copyright holder nor the names of its
//   contributors may be used to endorse or promote products derived from
//   this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Questions? Contact Michael A. Heroux (maherou@sandia.gov)
//
// ************************************************************************

#include "ddot.hpp"

/**
 * A method to compute the dot product of two vectors.
 *
 * This function optimises caching by only accessing one of the vectors if both of the
 * input values point to the same vector.
 *
 * @param width The number of vector elements (on this processor).
 * @param lhs The first input vector.
 * @param rhs The second input vector.
 * @param result A pointer to a scalar value, which is updated to contain the calculated dot
 * product.
 * @param time_allreduce A mutable variable tracking the time spent by MPI's reduce function.
 * @return An exit code of zero on success.
 */
int ddot(const int width, const double* const lhs, const double* const rhs, double* const result,
         double& time_allreduce) {
    double local_result = 0.0;
    if (rhs == lhs)
#ifdef USING_OMP
#pragma omp parallel for reduction(+ : local_result)
#endif
        for (int i = 0; i < width; i++) local_result += lhs[i] * lhs[i];
    else
#ifdef USING_OMP
#pragma omp parallel for reduction(+ : local_result)
#endif
        for (int i = 0; i < width; i++) local_result += lhs[i] * rhs[i];

#ifdef USING_MPI
    // Use MPI's reduce function to collect all partial sums
    double t0 = mytimer();
    double global_result = 0.0;
    MPI_Allreduce(&local_result, &global_result, 1, MPI_DOUBLE, MPI_SUM, MPI_COMM_WORLD);
    *result = global_result;
    time_allreduce += mytimer() - t0;
#else
    *result = local_result;
#endif

    return (0);
}
