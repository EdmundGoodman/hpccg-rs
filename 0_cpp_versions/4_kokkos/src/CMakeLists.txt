# Ensure the exectuables are outputted to the project base directory
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${PROJECT_SOURCE_DIR}")

## Compile the library source files
set(SOURCE_FILES
        generate_matrix.cpp read_HPC_row.cpp compute_residual.cpp mytimer.cpp
        dump_matlab_matrix.cpp HPC_sparsemv.cpp HPCCG.cpp waxpby.cpp ddot.cpp
        make_local_matrix.cpp exchange_externals.cpp YAML_Element.cpp YAML_Doc.cpp
)
add_library(HPCCG ${SOURCE_FILES})

# If the environment variable is set, build with Kokkos
if ($ENV{USE_KOKKOS})
    find_package(Kokkos REQUIRED)
    target_link_libraries(HPCCG PUBLIC Kokkos::kokkos)
    target_compile_options(HPCCG PUBLIC -DUSING_KOKKOS)
endif()

# If the environment variable is set, build with MPI
if ($ENV{USE_MPI})
    find_package(MPI REQUIRED)
    message(STATUS "Run: ${MPIEXEC} ${MPIEXEC_NUMPROC_FLAG} ${MPIEXEC_MAX_NUMPROCS} ${MPIEXEC_PREFLAGS} EXECUTABLE ${MPIEXEC_POSTFLAGS} ARGS")
    target_link_libraries(HPCCG PUBLIC MPI::MPI_CXX)
    target_compile_options(HPCCG PUBLIC -DUSING_MPI)
endif()

# Set debugging compile options
if(CMAKE_BUILD_TYPE MATCHES "Debug")
    target_compile_options(HPCCG PUBLIC -DDEBUG)
    target_compile_options(HPCCG PUBLIC -Wall -Wextra)
    target_compile_options(HPCCG PUBLIC -g -fno-omit-frame-pointer)
    target_compile_options(HPCCG PUBLIC -fsanitize=address -fsanitize=undefined)
    target_link_options(HPCCG PUBLIC -fsanitize=address -fsanitize=undefined)
endif()

# Print the compiler options set for debugging
get_target_property(HPCCG_lib_CFLAGS HPCCG COMPILE_OPTIONS)
message(STATUS "CXX HPCCG library flags: ${HPCCG_lib_CFLAGS}")

## Compile the executable source files
add_executable (test_HPCCG main.cpp)
target_link_libraries (test_HPCCG HPCCG)
