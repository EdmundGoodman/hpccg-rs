#include <catch2/catch_test_macros.hpp>

#include "../src/waxpby.hpp"

TEST_CASE("Test scaled vector addition implementation") {
    int width = 3;
    double vx[] = {1.0, 2.0, 3.0};
    double vy[] = {3.0, 2.0, 1.0};
    double alpha = 4.0;
    double beta = 5.0;
    double result[width];

    SECTION("Test neither alpha nor beta being 1.0") {
        double expected[] = {19.0, 18.0, 17.0};
        waxpby(width, alpha, vx, beta, vy, result);
        for (int i = 0; i < width; i++) REQUIRE(result[i] == expected[i]);
    }

    SECTION("Test alpha being 1.0") {
        alpha = 1.0;
        double expected[] = {16.0, 12.0, 8.0};
        waxpby(width, alpha, vx, beta, vy, result);
        for (int i = 0; i < width; i++) REQUIRE(result[i] == expected[i]);
    }

    SECTION("Test beta being 1.0") {
        alpha = 4.0;
        beta = 1.0;
        waxpby(width, alpha, vx, beta, vy, result);
        double expected[] = {7.0, 10.0, 13.0};
        for (int i = 0; i < width; i++) REQUIRE(result[i] == expected[i]);
    }
}
