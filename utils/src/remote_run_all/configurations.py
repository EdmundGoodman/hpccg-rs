#!/usr/bin/env python3
"""A set of functions to get different test configurations."""

from pathlib import Path
from collections.abc import Iterator

from test_configuration import TestConfiguration

RUST_BUILD_COMMAND = "cargo build --release"
CPP_BUILD_COMMAND = "make"

RUST_EXECUTABLE_PREFIX = Path("./target/release/")
BUILD_PARENT_DIRECTORY = Path("../")

ORIGINAL: tuple[Path, str, Path] = (
    BUILD_PARENT_DIRECTORY / "0_original",
    CPP_BUILD_COMMAND,
    Path("./test_HPCCG")
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


def _compare_translations_test_suite(
    run_args: list[str],
    cpu_count: int,
    timeout: str,
    memory_mb: int
) -> Iterator[TestConfiguration]:
    run_configs = [ORIGINAL] + TRANSLATIONS
    for run_arg in run_args:
        for (directory, build, executable) in run_configs:
            yield TestConfiguration(
                directory, build, executable, run_arg, cpu_count, timeout, memory_mb
            )


def _strong_scaling_test_suite(
    directory: Path,
    build: str,
    executable: Path,
    timeout: str,
    memory_mb: int
) -> Iterator[TestConfiguration]:
    cpu_counts = [2**i for i in range(0,7)]
    run_args = [f"64 64 {2**i}" for i in reversed(range(4,11))]
    for cpu_count, run_arg in zip(cpu_counts, run_args):
        yield TestConfiguration(
            directory, build, executable, run_arg, cpu_count, timeout, memory_mb
        )


def _weak_scaling_test_suite(
        directory: Path,
        build: str,
        executable: Path,
        timeout: str,
        memory_mb: int
) -> Iterator[TestConfiguration]:
    cpu_counts = [2**i for i in range(0,7)]
    run_args = [f"64 64 {2**i}" for i in reversed(range(4,11))]
    for cpu_count, run_arg in zip(cpu_counts, run_args):
        yield TestConfiguration(
            directory, build, executable, run_arg, cpu_count, timeout, memory_mb
        )


def compare_translations_test_suite() -> Iterator[TestConfiguration]:
    cpu_count = 40
    timeout = "60:00"
    memory_mb = 60_000
    run_args = [f"{x} {x} {x}" for x in range(50, 401, 50)]
    yield from _compare_translations_test_suite(
        run_args, cpu_count, timeout, memory_mb
    )


def strong_scaling_test_suite() -> Iterator[TestConfiguration]:
    timeout = "60:00"
    memory_mb = 60_000
    yield from _strong_scaling_test_suite(*ORIGINAL, timeout, memory_mb)


def weak_scaling_test_suite() -> Iterator[TestConfiguration]:
    timeout = "60:00"
    memory_mb = 60_000
    yield from _weak_scaling_test_suite(*ORIGINAL, timeout, memory_mb)