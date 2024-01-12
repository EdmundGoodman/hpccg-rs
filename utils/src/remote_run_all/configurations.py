#!/usr/bin/env python3
"""A set of functions to get different test configurations."""

from collections.abc import Iterator
from pathlib import Path

from test_configuration import TestConfiguration

RUST_BUILD_COMMAND = "cargo build --release"
CPP_BUILD_COMMAND = "make"

RUST_EXECUTABLE_PREFIX = Path("./target/release/")
BUILD_PARENT_DIRECTORY = Path("../")

ORIGINAL: tuple[Path, str, Path] = (
    BUILD_PARENT_DIRECTORY / "0_original",
    CPP_BUILD_COMMAND,
    Path("./test_HPCCG"),
)

TRANSLATIONS: list[tuple[Path, str, Path]] = [
    (
        BUILD_PARENT_DIRECTORY / "1_naive",
        RUST_BUILD_COMMAND,
        RUST_EXECUTABLE_PREFIX / "hpccg-rs-naive",
    ),
    (
        BUILD_PARENT_DIRECTORY / "2_indexed",
        RUST_BUILD_COMMAND,
        RUST_EXECUTABLE_PREFIX / "hpccg-rs-indexed",
    ),
    (
        BUILD_PARENT_DIRECTORY / "3_single_indexed",
        RUST_BUILD_COMMAND,
        RUST_EXECUTABLE_PREFIX / "hpccg-rs-single-indexed",
    ),
    (
        BUILD_PARENT_DIRECTORY / "4_no_bounds_check",
        RUST_BUILD_COMMAND,
        RUST_EXECUTABLE_PREFIX / "hpccg-rs-no-bounds-check",
    ),
    (
        BUILD_PARENT_DIRECTORY / "5_iterators",
        RUST_BUILD_COMMAND,
        RUST_EXECUTABLE_PREFIX / "hpccg-rs-iterators",
    ),
    (
        BUILD_PARENT_DIRECTORY / "6_parallel",
        RUST_BUILD_COMMAND,
        RUST_EXECUTABLE_PREFIX / "hpccg-rs-parallel",
    ),
]


def generate_compare_translations_test_suite(
    run_args: list[str], cpu_count: int, timeout: str, memory_mb: int
) -> Iterator[TestConfiguration]:
    """Get an iterator over a specified test suite for comparing translations."""
    for run_arg in run_args:
        for directory, build, executable in [ORIGINAL, *TRANSLATIONS]:
            yield TestConfiguration(
                directory, build, executable, run_arg, cpu_count, timeout, memory_mb
            )


def generate_strong_scaling_test_suite(
    directory: Path, build: str, executable: Path, timeout: str, memory_mb: int
) -> Iterator[TestConfiguration]:
    """Get an iterator over a specified test suite for strong scaling."""
    cpu_counts = [2**i for i in range(0, 7)]
    run_args = [f"64 64 {2**i}" for i in reversed(range(4, 11))]
    for cpu_count, run_arg in zip(cpu_counts, run_args):
        yield TestConfiguration(
            directory, build, executable, run_arg, cpu_count, timeout, memory_mb
        )


def generate_weak_scaling_test_suite(
    directory: Path, build: str, executable: Path, timeout: str, memory_mb: int
) -> Iterator[TestConfiguration]:
    """Get an iterator over a specified test suite for weak scaling."""
    cpu_counts = [2**i for i in range(0, 7)]
    run_args = [f"64 64 {2**i}" for i in reversed(range(4, 11))]
    for cpu_count, run_arg in zip(cpu_counts, run_args):
        yield TestConfiguration(
            directory, build, executable, run_arg, cpu_count, timeout, memory_mb
        )
