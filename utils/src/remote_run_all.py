#!/usr/bin/env python3
"""A script to run a test matrix of hpccg translations on Kudu batch computer."""

from pathlib import Path
from subprocess import run as subprocess_run

RUN_ARGS: list[str] = [
    "50 50 50",
    "100 100 100",
    "150 150 150",
    "200 200 200",
    "250 250 250",
    "300 300 300",
    "350 350 350",
    "400 400 400",
]

# Data schema: (directory, build, executable, args)
RUN_COMMANDS: list[tuple[Path, str, Path]] = [
    (Path("../0_original"), "make", Path("./test_HPCCG")),
    (
        Path("../1_naive"),
        "cargo build --release",
        Path("./target/release/hpccg-rs-naive"),
    ),
    (
        Path("../2_indexed"),
        "cargo build --release",
        Path("./target/release/hpccg-rs-indexed"),
    ),
    (
        Path("../3_single_indexed"),
        "cargo build --release",
        Path("./target/release/hpccg-rs-single-indexed"),
    ),
    (
        Path("../4_no_bounds_check"),
        "cargo build --release",
        Path("./target/release/hpccg-rs-no-bounds-check"),
    ),
    (
        Path("../5_iterators"),
        "cargo build --release",
        Path("./target/release/hpccg-rs-iterators"),
    ),
    (
        Path("../6_parallel"),
        "cargo build --release",
        Path("./target/release/hpccg-rs-parallel"),
    ),
]


def main() -> None:
    """Run the test matrix."""
    for i, args in enumerate(RUN_ARGS):
        print(f"{i+1}) Running with arguments: {args}")
        for j, (directory, build, executable) in enumerate(RUN_COMMANDS):
            command: str = f"cd {directory} && {build} && time {executable} {args}"
            print(f"\t{j+1}) `{command}`")
            subprocess_run(["./remoterun.sh", command])  # noqa: S603
            print("")
        print("\n")


if __name__ == "__main__":
    main()
