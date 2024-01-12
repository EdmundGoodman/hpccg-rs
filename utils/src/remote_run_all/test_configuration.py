#!/usr/bin/env python3
"""A class for test configurations on batch compute."""

from subprocess import run as subprocess_run
from pathlib import Path
from textwrap import dedent
from dataclasses import dataclass
from tempfile import NamedTemporaryFile
from contextlib import chdir

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
        return dedent(f"""
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
        """[1:])

    def run(self) -> None:
        """Run the specified test on batch compute."""
        with chdir(Path(__file__).parent):
            with NamedTemporaryFile(suffix=".sbatch", dir=Path("./"), mode="w+") as sbatch_tmp:
                sbatch_tmp.write(self.generate_sbatch_file())
                sbatch_tmp.flush()
                subprocess_run(["./remoterun.sh", Path(sbatch_tmp.name)])
