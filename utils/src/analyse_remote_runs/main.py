#!/usr/bin/env python3
"""A script to analyse the runs resulting from the test matrix."""

from collections.abc import Iterator
from dataclasses import dataclass
from pathlib import Path
from re import search as re_search
from typing import Optional

import matplotlib.pyplot as plt
import seaborn as sns

plt.style.use("seaborn-v0_8")

RESULTS_DIRECTORY: Path = Path("src/analyse_remote_runs/all_runs")

NAME_REGEX = r"Mini-Application Name: ([a-zA-Z0-9_\-]*)"
DIMENSIONS_NAMES = ("nx", "ny", "nz")
DIMENSIONS_REGEX = "".join([name + r": (\d+)\s+" for name in DIMENSIONS_NAMES])
METRIC_NAMES = ("Total", "DDOT", "WAXPBY", "SPARSEMV")
TIMES_REGEX = r"Time Summary:\s+" + "".join(
    [name + r"\s*: ([\d\.]+)\s+" for name in METRIC_NAMES]
)
FLOPS_REGEX = r"FLOPS Summary:\s+" + "".join(
    [name + r"\s*: ([\d\.]+)\s+" for name in METRIC_NAMES]
)
MFLOPS_REGEX = r"MFLOPS Summary:\s+" + "".join(
    [name + r"\s*: ([\d\.]+)\s+" for name in METRIC_NAMES]
)


@dataclass
class RunResult:
    """A dataclass to represent the relevant data about a remote run."""

    name: str
    dimensions: tuple[int, ...]
    times: tuple[float, ...]
    flops: tuple[int, ...]
    mflops: tuple[float, ...]

    @classmethod
    def parse_test(cls, results_file: Path) -> Optional["RunResult"]:
        """Parse data about a specific test from its output file to a dataclass."""
        run_output = results_file.read_text(encoding="utf-8")
        # Name
        name_search = re_search(NAME_REGEX, run_output)
        if name_search is None:
            return None
        name = name_search.group(1)
        # Dimensions
        dimensions_search = re_search(DIMENSIONS_REGEX, run_output)
        assert dimensions_search is not None
        dimensions = tuple(
            int(dimensions_search.group(i + 1)) for i in range(len(DIMENSIONS_NAMES))
        )
        # Times
        times_search = re_search(TIMES_REGEX, run_output)
        assert times_search is not None
        times = tuple(
            float(times_search.group(i + 1)) for i in range(len(METRIC_NAMES))
        )
        # FLOPS
        flops_search = re_search(FLOPS_REGEX, run_output)
        # `int(float(...))` used to get out of scientific notation
        assert flops_search is not None
        flops = tuple(
            int(float(flops_search.group(i + 1))) for i in range(len(METRIC_NAMES))
        )
        # MFLOPS
        mflops_search = re_search(MFLOPS_REGEX, run_output)
        assert mflops_search is not None
        mflops = tuple(
            float(mflops_search.group(i + 1)) for i in range(len(METRIC_NAMES))
        )

        return cls(name, dimensions, times, flops, mflops)


def get_run_results() -> Iterator[RunResult | None]:
    """Get an iterator of dataclasses representing each of the valid runs."""
    for run_directory in RESULTS_DIRECTORY.iterdir():
        for results_file in run_directory.iterdir():
            if results_file.suffix == ".out":
                yield RunResult.parse_test(results_file)


def main() -> None:
    """Analyse the specified remote runs."""
    data_series: dict[str, list[tuple[int, float]]] = {}
    for run_result in get_run_results():
        size = run_result.dimensions[0]
        # Ignore some with missing data...
        if (
            run_result is None
            or run_result.dimensions[0] >= 350
            or "parallel" in run_result.name
        ):
            continue
        if run_result.name not in data_series:
            data_series[run_result.name] = []
        data_series[run_result.name].append((size, run_result.times[0]))

    for name, data in data_series.items():
        size, time = list(zip(*sorted(data, key=lambda a: a[0])))
        sns.lineplot(x=size, y=time, label=name.replace("-", "_"))
    plt.xlabel("Size []")
    plt.ylabel("Time [s]")
    plt.legend()
    plt.show()


if __name__ == "__main__":
    main()
