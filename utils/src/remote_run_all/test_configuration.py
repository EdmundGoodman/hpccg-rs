#!/usr/bin/env python3
"""A class for test configurations on batch compute."""

from contextlib import chdir
from dataclasses import dataclass
from pathlib import Path
from subprocess import run as subprocess_run
from tempfile import NamedTemporaryFile
from textwrap import dedent


@dataclass
class TestConfiguration:
    """A dataclass representing a batch compute test job."""

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
        return dedent(
            f"""
        #!/bin/sh
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
        """[
                1:
            ]
        )

    def run(self) -> None:
        """Run the specified test on batch compute."""
        with chdir(Path(__file__).parent), NamedTemporaryFile(
            suffix=".sbatch", dir=Path("./"), mode="w+"
        ) as sbatch_tmp:
            sbatch_tmp.write(self.generate_sbatch_file())
            sbatch_tmp.flush()
            subprocess_run(["./remoterun.sh", Path(sbatch_tmp.name)])  # noqa: S603
