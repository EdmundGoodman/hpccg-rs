MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.DEFAULT_GOAL := default

.PHONY: clean
clean:
	rm -rf joboutputs

.PHONY: default
default:
	./remoterun.sh

.PHONY: 1_naive
1_naive:
	./remoterun.sh ../1_naive/Cargo.toml

.PHONY: 2_indexed
2_indexed:
	./remoterun.sh ../2_indexed/Cargo.toml

.PHONY: 3_single_indexed
3_single_indexed:
	./remoterun.sh ../3_single_indexed/Cargo.toml

.PHONY: 4_no_bounds_check
4_no_bounds_check:
	./remoterun.sh ../4_no_bounds_check/Cargo.toml

.PHONY: 5_iterators
5_iterators:
	./remoterun.sh ../5_iterators/Cargo.toml

.PHONY: 6_parallel
6_parallel:
	./remoterun.sh ../6_parallel/Cargo.toml

.PHONY: all
all: 1_naive 2_indexed 3_single_indexed 4_no_bounds_check 5_iterators 6_parallel