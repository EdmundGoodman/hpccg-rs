#!/usr/bin/env python3
"""A script to run a test matrix of hpccg translations on Kudu batch computer."""

from configurations import compare_translations_test_suite
# from configurations import strong_scaling_test_suite
# from configurations import weak_scaling_test_suite


def main() -> None:
    """Run the test matrix."""
    for test in compare_translations_test_suite():
        print(f"Starting {test.directory.name} @ '{test.args}' ({test.cpu_count} cores, {test.memory_mb/1000}GB RAM)")
        test.run()
        print("\n")


if __name__ == "__main__":
    main()
