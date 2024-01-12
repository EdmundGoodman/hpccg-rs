#!/usr/bin/env python3
"""A script to run a test matrix of hpccg translations on Kudu batch computer."""

from collections.abc import Iterator

from configurations import (
    ORIGINAL,
    generate_compare_translations_test_suite,
    generate_strong_scaling_test_suite,
    generate_weak_scaling_test_suite,
    TRANSLATIONS
)
from test_configuration import TestConfiguration


def compare_translations_test_suite() -> Iterator[TestConfiguration]:
    """Get an iterator over a configurable test suite for comparing translations."""
    cpu_count = 40
    timeout = "60:00"
    memory_mb = 60_000
    run_args = [f"{x} {x} {x}" for x in range(50, 401, 50)]
    yield from generate_compare_translations_test_suite(
        run_args, cpu_count, timeout, memory_mb
    )


def strong_scaling_test_suite() -> Iterator[TestConfiguration]:
    """Get an iterator over a configurable test suite for strong scaling."""
    timeout = "60:00"
    memory_mb = 60_000
    for directory, build, executable in [ORIGINAL, *TRANSLATIONS[-2:]]:
        yield from generate_strong_scaling_test_suite(
            directory, build, executable, timeout, memory_mb
        )


def weak_scaling_test_suite() -> Iterator[TestConfiguration]:
    """Get an iterator over a configurable test suite for weak scaling."""
    timeout = "60:00"
    memory_mb = 60_000
    for directory, build, executable in [ORIGINAL, *TRANSLATIONS[-2:]]:
        yield from generate_weak_scaling_test_suite(
            directory, build, executable, timeout, memory_mb
        )


def main() -> None:
    """Run a test suite."""
    for test in weak_scaling_test_suite():
        print(
            f"Starting {test.directory.name} @ '{test.args}' "
            f"({test.cpu_count} cores, {test.memory_mb/1000}GB RAM)"
        )
        test.run()
        print("\n")


if __name__ == "__main__":
    main()
