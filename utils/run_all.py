#!/usr/bin/env python3

from typing import List, Tuple
from pathlib import Path
from subprocess import run as subprocess_run

RUN_ARGS: List[str] = [
    "25 25 25",
    # "125 125 125",
]

# (directory, build, executable, args)
RUN_COMMANDS: List[Tuple[Path, str, Path]] = [
    (Path(".."), "cargo build --release", Path("./target/release/hpccg-rs")),
    (Path("../0_original"), "make", Path("./test_HPCCG")),
    #(Path("../1_naive"), "cargo build --release", Path("./target/release/hpccg-rs-naive")),
    #(Path("../2_indexed"), "cargo build --release", Path("./target/release/hpccg-rs-indexed")),
    #(Path("../3_single_indexed"), "cargo build --release", Path("./target/release/hpccg-rs-single-indexed")),
    #(Path("../4_no_bounds_check"), "cargo build --release", Path("./target/release/hpccg-rs-no-bounds-check")),
    #(Path("../5_iterators"), "cargo build --release", Path("./target/release/hpccg-rs-iterators")),
    #(Path("../6_parallel"), "cargo build --release", Path("./target/release/hpccg-rs-parallel")),
]

def main() -> None:
    for i, args in enumerate(RUN_ARGS):
        print(f"{i+1}) Running with arguments: {args}")
        for j, (directory, build, executable) in enumerate(RUN_COMMANDS):
            command: str = f"cd {directory} && {build} && {executable} {args}"
            print(f"\t{j+1}) `{command}`")
            subprocess_run(["./remoterun.sh", command])

if __name__ == "__main__":
    main()
