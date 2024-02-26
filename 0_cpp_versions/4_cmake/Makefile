# Makefile configuration
MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.DEFAULT_GOAL := all

# Project configuration
TARGET = test_HPCCG
TEST_TARGET = Catch2Tests
CMAKE_BUILD_DIR = build
export CC=/usr/bin/g++
export CXX=/usr/bin/g++
# NOTE: You will need to clean build when changing these values
export USE_OMP=1
export USE_MPI=1
# export DEBUG=1

# Build targets
.PHONY: all
all: $(TARGET)

$(CMAKE_BUILD_DIR):
	cmake -S . -B build -DCMAKE_EXPORT_COMPILE_COMMANDS=1

$(TARGET): $(CMAKE_BUILD_DIR)
	cmake --build $(CMAKE_BUILD_DIR) --target $(TARGET) -j 6

# Utility targets
.PHONY: clean
clean:
	rm -rf $(TARGET) $(CMAKE_BUILD_DIR)

.PHONY: no_yaml
no_yaml:
	@rm -f *.yaml

.PHONY: format
format:
	@clang-format -style=file -i src/*.cpp src/*.hpp test/*.cpp test/*.hpp
