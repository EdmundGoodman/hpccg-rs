#!/usr/bin/env python3
"""A script to run a test matrix of hpccg translations on Kudu batch computer."""
from subprocess import run as subprocess_run
from pathlib import Path
from textwrap import dedent
from dataclasses import dataclass
from tempfile import NamedTemporaryFile
from collections.abc import Iterator
from tqdm import tqdm

# Data schema: (directory, build_command, run_command)
RUST_BUILD_COMMAND = "cargo build --release"
CPP_BUILD_COMMAND = "make"
RUST_EXECUTABLE_PREFIX = Path("./target/release/")
BUILD_PARENT_DIRECTORY = Path("../")
RUN_CONFIG: list[tuple[Path, str, Path]] = [
    (
        BUILD_PARENT_DIRECTORY / "0_original",
        CPP_BUILD_COMMAND,
        Path("./test_HPCCG")
    ),
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
][:2]

RUN_ARGS: list[str] = [
    "50 50 50",
    #"100 100 100",
    #"150 150 150",
    #"200 200 200",
    #"250 250 250",
    #"300 300 300",
    #"350 350 350",
    #"400 400 400",
]

BATCH_CONFIG: list[tuple[int, str, int]] = [
    (40, "10:00", 60_000),
]

@dataclass
class TestConfiguration:

    directory: Path
    build_command: str
    run_command: Path
    args: str
    cpu_count: int
    timeout: str
    memory_mb: int
    output_prefix: str = "%j/dissertation"

    def generate_sbatch_file(self) -> str:
        """Create a .sbatch file from the test's configuration."""
        return dedent(f"""#!/bin/sh
        #SBATCH --job-name=multicore-cpu
        #SBATCH --partition=cpu-batch
        #SBATCH --cpus-per-task={self.cpu_count}
        #SBATCH --time={self.timeout}
        #SBATCH --mem={self.memory_mb}
        #SBATCH --exclusive=mcs
        #SBATCH --output={self.output_prefix}_output_%j.out
        #SBATCH --error={self.output_prefix}_error_%j.err
        echo "===== ENVIRONMENT ====="
        lscpu
        echo
        echo "===== RUN RESULTS ====="
        cd {self.directory}
        {self.build_command}
        time ./{self.run_command} {self.args}
        """)

    def run(self) -> None:
        """Run the specified test on batch compute."""
        with NamedTemporaryFile(suffix=".sbatch", dir=Path.cwd(), mode="w+") as sbatch_tmp:
            sbatch_tmp.write(self.generate_sbatch_file())
            sbatch_tmp.flush()
            subprocess_run(["./remoterun.sh", Path(sbatch_tmp.name)])
            # subprocess_run(["echo", Path(sbatch_tmp.name)])


def get_test_suite() -> Iterator[TestConfiguration]:
    """Yield an iterator over the test suite."""
    for (cpu_count, timeout, memory_mb) in BATCH_CONFIG:
        for args in RUN_ARGS:
            for (directory, build, executable) in RUN_CONFIG:
                yield TestConfiguration(
                    directory, build, executable, args, cpu_count, timeout, memory_mb
                )


def main() -> None:
    """Run the test matrix."""
    for test in (progress_bar := tqdm(get_test_suite(), leave=True)):
        progress_bar.set_description(
            f"Starting {test.directory.name} @ '{test.args}' ({test.cpu_count} cores, {test.memory_mb/1000}GB RAM)"
        )
        test.run()


if __name__ == "__main__":
    main()
