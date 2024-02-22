#include <catch2/catch_test_macros.hpp>

#include "../src/compute_residual.hpp"

TEST_CASE("Test compute residual implementation") {
    int width = 3;
    double lhs[] = {1.0, 2.0, 3.0};
    double rhs[] = {3.0, 2.0, 1.0};
    double result;
    compute_residual(width, lhs, rhs, &result);
    REQUIRE(result == 2.0);
}
