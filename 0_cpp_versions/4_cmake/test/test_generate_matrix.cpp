#include <catch2/catch_test_macros.hpp>
#include <cmath>

#include "../src/HPC_Sparse_Matrix.hpp"
#include "../src/generate_matrix.hpp"

TEST_CASE("Test sparse matrix generation") {
    int nx = 2;
    int ny = 2;
    int nz = 2;
    HPC_Sparse_Matrix *A;
    double *x, *b, *xexact;
    generate_matrix(nx, ny, nz, &A, &x, &b, &xexact);

    SECTION("Check matrix size") {
        REQUIRE(A->local_nrow == 8);
        REQUIRE(A->local_nnz == 216);
    }

    SECTION("Check number of non-zeroes") {
        for (int i = 0; i < A->local_nrow; i++) REQUIRE(A->nnz_in_row[i] == A->local_nrow);
    }

    SECTION("Check matrix data") {
        double expected_val[] = {
            27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 27.0, -1.0, -1.0, -1.0,
            -1.0, -1.0, -1.0, -1.0, -1.0, 27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
            -1.0, 27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 27.0, -1.0, -1.0,
            -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
            -1.0, -1.0, 27.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 27.0,
        };
        REQUIRE(pow(A->local_nrow, 2) == sizeof(expected_val) / sizeof(double));
        for (int i = 0; i < pow(A->local_nrow, 2); i++)
            REQUIRE(A->list_of_vals[i] == expected_val[i]);
    }

    SECTION("Check auxiliary generated data") {
        for (int i = 0; i < A->local_nrow; i++) REQUIRE(x[i] == 0.0);
        for (int i = 0; i < A->local_nrow; i++) REQUIRE(b[i] == 20.0);
        for (int i = 0; i < A->local_nrow; i++) REQUIRE(xexact[i] == 1.0);
    }
}
