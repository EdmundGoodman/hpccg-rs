# Makefile configuration
MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.DEFAULT_GOAL := all

# Project configuration
TARGET = test_HPCCG
TEST_TARGET = Catch2Tests
CMAKE_BUILD_DIR = build
export CC=/usr/bin/clang++
export CXX=/usr/bin/clang++
# NOTE: You will need to clean build when changing these values
# export USE_MPI=1
export USE_KOKKOS=1
# export DEBUG=1

# Build targets
.PHONY: all
all: clean $(TARGET)

$(CMAKE_BUILD_DIR):
	cmake -S . -B build -DCMAKE_EXPORT_COMPILE_COMMANDS=1

$(TARGET): $(CMAKE_BUILD_DIR)
	cmake --build $(CMAKE_BUILD_DIR) --target $(TARGET) -j 6

.PHONY: test
test: $(CMAKE_BUILD_DIR)
	cmake --build $(CMAKE_BUILD_DIR) --target $(TEST_TARGET) -j 6
	./$(CMAKE_BUILD_DIR)/test/$(TEST_TARGET) --success

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
