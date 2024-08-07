# Ensure the exectuables are outputted to the project base directory
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "${PROJECT_SOURCE_DIR}")

## Compile the library source files
set(SOURCE_FILES
        generate_matrix.cpp read_HPC_row.cpp compute_residual.cpp mytimer.cpp
        dump_matlab_matrix.cpp HPC_sparsemv.cpp HPCCG.cpp waxpby.cpp ddot.cpp
        make_local_matrix.cpp exchange_externals.cpp YAML_Element.cpp YAML_Doc.cpp
)
add_library(HPCCG ${SOURCE_FILES})

# If the environment variable is set, build with OpenMP
if ($ENV{USE_OMP})
    find_package(OpenMP)
    if(OpenMP_CXX_FOUND)
        target_link_libraries(HPCCG PUBLIC OpenMP::OpenMP_CXX)
    endif()
    target_compile_options(HPCCG PUBLIC -DUSING_OMP)
endif()

# If the environment variable is set, build with MPI
if ($ENV{USE_MPI})
    find_package(MPI REQUIRED)
    message(STATUS "Run: ${MPIEXEC} ${MPIEXEC_NUMPROC_FLAG} ${MPIEXEC_MAX_NUMPROCS} ${MPIEXEC_PREFLAGS} EXECUTABLE ${MPIEXEC_POSTFLAGS} ARGS")
    target_link_libraries(HPCCG PUBLIC MPI::MPI_CXX)
    target_compile_options(HPCCG PUBLIC -DUSING_MPI)
endif()

# Set equivalent compiler optimisation flags
target_compile_options(HPCCG PUBLIC -ftree-vectorize -ftree-vectorizer-verbose=2)
# target_compile_options(HPCCG PUBLIC -Q --help=optimizers)

# Print the compiler options set for debugging
get_target_property(HPCCG_lib_CFLAGS HPCCG COMPILE_OPTIONS)
message(STATUS "CXX HPCCG library flags: ${HPCCG_lib_CFLAGS}")

## Compile the executable source files
add_executable (test_HPCCG main.cpp)
target_link_libraries (test_HPCCG HPCCG)
