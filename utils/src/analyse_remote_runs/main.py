#!/usr/bin/env python3
"""A script to analyse the runs resulting from the test matrix."""

from typing import Self, Optional
from pathlib import Path
from re import search as re_search
from dataclasses import dataclass

RESULTS_DIRECTORY: Path = Path("src/analyse_remote_runs/all_runs")

NAME_REGEX = r"Mini-Application Name: ([a-zA-Z0-9_\-]*)"
DIMENSIONS_NAMES = ("nx", "ny", "nz")
DIMENSIONS_REGEX = "".join([name + r": (\d+)\s+" for name in DIMENSIONS_NAMES])
METRIC_NAMES = ("Total", "DDOT", "WAXPBY", "SPARSEMV")
TIMES_REGEX = r"Time Summary:\s+" + "".join([name + r"\s*: ([\d\.]+)\s+" for name in METRIC_NAMES])
FLOPS_REGEX = r"FLOPS Summary:\s+" + "".join([name + r"\s*: ([\d\.]+)\s+" for name in METRIC_NAMES])
MFLOPS_REGEX = r"MFLOPS Summary:\s+" + "".join([name + r"\s*: ([\d\.]+)\s+" for name in METRIC_NAMES])


@dataclass
class RunResult:
    name: str
    dimensions: tuple[int, ...]
    times: tuple[float, ...]
    flops: tuple[int, ...]
    mflops: tuple[float, ...]

    @classmethod
    def parse_test(cls: Self, results_file: Path) -> Optional[Self]:
        """Parse data about a specific test from its output file to a dataclass."""
        run_output = results_file.read_text(encoding="utf-8")
        # Name
        name_search = re_search(NAME_REGEX, run_output)
        if name_search is None:
            return None
        name = name_search.group(1)
        # Dimensions
        dimensions_search = re_search(DIMENSIONS_REGEX, run_output)
        dimensions = tuple(int(dimensions_search.group(i+1)) for i in range(len(DIMENSIONS_NAMES)))
        # Times
        times_search = re_search(TIMES_REGEX, run_output)
        times = tuple(float(times_search.group(i+1)) for i in range(len(METRIC_NAMES)))
        # FLOPS
        flops_search = re_search(FLOPS_REGEX, run_output)
        # `int(float(...))` used to get out of scientific notation
        flops = tuple(int(float(flops_search.group(i+1))) for i in range(len(METRIC_NAMES)))
        # MFLOPS
        mflops_search = re_search(MFLOPS_REGEX, run_output)
        mflops = tuple(float(mflops_search.group(i+1)) for i in range(len(METRIC_NAMES)))

        return cls(name, dimensions, times, flops, mflops)


def main() -> None:
    """Analysis the runs from a test matrix."""
    # Walk the results directory tree
    # For each file, pattern match to characterise it and get its data
    for run_directory in RESULTS_DIRECTORY.iterdir():
        for results_file in run_directory.iterdir():
            if results_file.suffix == ".out":
                result = RunResult.parse_test(results_file)
                if result is None:
                    print(f"Expected output missing - skipping file '{results_file}'!")
                else:
                    print(result)


if __name__ == "__main__":
    main()
