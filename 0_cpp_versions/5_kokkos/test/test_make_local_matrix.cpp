#include <catch2/catch_test_macros.hpp>
#include <cmath>

#include "../src/HPC_Sparse_Matrix.hpp"
#include "../src/generate_matrix.hpp"

TEST_CASE("Test sparse matrix generation") {
#ifdef USING_MPI
    int nx = 2;
    int ny = 2;
    int nz = 2;
    HPC_Sparse_Matrix *A;
    double *x, *b, *xexact;
    generate_matrix(nx, ny, nz, &A, &x, &b, &xexact);

    make_local_matrix(&A);
#else
    SECTION("`make_local_matrix` not defined when not in MPI mode") { REQUIRE(1 == 1); }
#endif
}
