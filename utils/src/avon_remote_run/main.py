#!/usr/bin/env python3
"""A class for test configurations on batch compute."""

from dataclasses import dataclass
from pathlib import Path
from subprocess import run as subprocess_run
from tempfile import NamedTemporaryFile
from re import search as re_search

JOB_ID_REGEX = r"Submitted batch job (\d+)"


@dataclass
class RunConfiguration:
    """A dataclass representing a batch compute job."""

    directory: Path
    build_command: str
    run_command: Path
    args: str
    sbatch_config: dict[str, str]
    modules: list[str]

    def generate_sbatch_file(self) -> str:
        """Create a .sbatch file from the run's configuration."""
        sbatch_file = "#!/bin/sh\n"

        for key, value in self.sbatch_config.items():
            sbatch_file += f"#SBATCH --{key}={value}\n"

        if len(self.modules) > 0:
            sbatch_file += "\nmodule purge\n"
            sbatch_file += f"module load {' '.join(self.modules)}\n"

        sbatch_file += "\necho '===== ENVIRONMENT ====='\n"
        sbatch_file += "lscpu\n"
        sbatch_file += "echo\n"

        sbatch_file += "\necho '===== BUILD & RUN ====='\n"
        sbatch_file += f"cd {self.directory}\n"
        sbatch_file += f"{self.build_command}\n"
        sbatch_file += f"time srun ./{self.run_command} {self.args}\n"

        return sbatch_file

    def run(self) -> int | None:
        """Run the specified test on batch compute."""
        with NamedTemporaryFile(suffix=".sbatch", dir=Path("./"), mode="w+") as sbatch_tmp:
            sbatch_tmp.write(self.generate_sbatch_file())
            sbatch_tmp.flush()
            result = subprocess_run(["sbatch", Path(sbatch_tmp.name)], capture_output=True, text=True)  # noqa: S603
            job_id_search = re_search(JOB_ID_REGEX, str(result.output))
            if job_id_search is None:
                return None
            return int(job_id_search.group(1))


if __name__ == "__main__":
    foo = RunConfiguration(
        directory=Path("../"),
        build_command="make -j 8",
        run_command=Path("./test_HPCCG"),
        args="50 50 50",
        sbatch_config={
            "cpus-per-task":"48"
        },
        modules=["GCC/11.3.0", "OpenMPI/4.1.4"]
    )
    print(foo.generate_sbatch_file())
