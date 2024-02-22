#include <catch2/catch_test_macros.hpp>

#include "../src/HPC_Sparse_Matrix.hpp"
#include "../src/HPC_sparsemv.hpp"
#include "../src/generate_matrix.hpp"

TEST_CASE("Test sparse matrix-vector multiplication implementation") {
    int nx = 2;
    int ny = 2;
    int nz = 2;
    HPC_Sparse_Matrix *A;
    double *x, *b, *xexact;
    generate_matrix(nx, ny, nz, &A, &x, &b, &xexact);

    double vector[] = {20.0, 20.0, 20.0, 20.0, 20.0, 20.0, 20.0, 20.0, 20.0};
    double result[7];
    double expected[] = {400.0, 400.0, 400.0, 400.0, 400.0, 400.0, 400.0, 400.0};

    HPC_sparsemv(A, vector, result);
    for (int i = 0; i < 7; i++) REQUIRE(result[i] == expected[i]);
}
