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
	./remoterun.sh ../1_naive/Cargo.toml hpccg-rs-naive

.PHONY: 2_indexed
2_indexed:
	./remoterun.sh ../2_indexed/Cargo.toml hpccg-rs-indexed

.PHONY: 3_single_indexed
3_single_indexed:
	./remoterun.sh ../3_single_indexed/Cargo.toml hpccg-rs-single-indexed

.PHONY: 4_no_bounds_check
4_no_bounds_check:
	./remoterun.sh ../4_no_bounds_check/Cargo.toml hpccg-rs-no-bounds-check

.PHONY: 5_iterators
5_iterators:
	./remoterun.sh ../5_iterators/Cargo.toml hpccg-rs-iterators

.PHONY: 6_parallel
6_parallel:
	./remoterun.sh ../6_parallel/Cargo.toml hpccg-rs-parallel

.PHONY: all
all: 1_naive 2_indexed 3_single_indexed 4_no_bounds_check 5_iterators 6_parallel
