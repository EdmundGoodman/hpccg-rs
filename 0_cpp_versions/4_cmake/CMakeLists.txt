cmake_minimum_required(VERSION 3.26)
project(
        HPCCG
        VERSION 1.1.0
        DESCRIPTION "High Performance Computing Conjugate Gradients: The original Mantevo miniapp."
        LANGUAGES CXX
)
set(CMAKE_CXX_STANDARD 17)

# Specify global C++ compiler flags
add_compile_options(-g -O3)

# Set Debug mode if an environment variable is set
if($ENV{DEBUG})
    set(CMAKE_BUILD_TYPE Debug)
endif()
message(STATUS "CMake build type: ${CMAKE_BUILD_TYPE}")

# Build the source directory
add_subdirectory(src)
