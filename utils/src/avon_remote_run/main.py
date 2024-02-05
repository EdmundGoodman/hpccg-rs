#!/usr/bin/env python3
"""A class for test configurations on batch compute."""

from pathlib import Path
from subprocess import run as subprocess_run
from subprocess import PIPE as subprocess_PIPE
from tempfile import NamedTemporaryFile
from re import search as re_search

JOB_ID_REGEX = r"Submitted batch job (\d+)"

# TODO: Switch to setting attributes then constructing with a getter on
# sbatch contents to enforce order?
class RunConfiguration:
    """A builder/runner for a run configuration"""

    def __init__(self):
        """Initialise the run configuration file as a empty bash file."""
        self.sbatch_contents = "#!/bin/sh\n"

    def add_sbatch_config(self, config: dict[str, str]) -> None:
        """Add sbatch flags to the run configuration file."""
        for key, value in config.items():
            self.sbatch_contents += f"#SBATCH --{key}={value}\n"

    def add_module_loads(self, modules: list[str]) -> None:
        """Add module loads to the run configuration file."""
        self.sbatch_contents += "\nmodule purge\n"
        self.sbatch_contents += f"module load {' '.join(modules)}\n"

    def add_display_environment(self) -> None:
        """Add displaying the environment to the run configuration file."""
        self.sbatch_contents += "\necho '===== ENVIRONMENT ====='\n"
        self.sbatch_contents += "lscpu\necho\n"

    def add_build_step(
        self, commands: list[str], directory: Path | None = None
    ) -> None:
        """Add build commands to the run configuration file."""
        self.sbatch_contents += "\necho '===== BUILD ====='\n"
        if directory is not None:
            self.sbatch_contents += f"cd {directory}\n"
        self.sbatch_contents += "\n".join(commands) + "\n"

    def add_run_step(self, command: str, args: str | None = None) -> None:
        """Add run commands to the run configuration file."""
        self.sbatch_contents += "\necho '===== RUN ====='\n"
        self.sbatch_contents += f"time srun {command}"
        if args is not None:
            self.sbatch_contents += f" {args}"
        self.sbatch_contents += "\n"

    def __repr__(self) -> str:
        """Get the sbatch configuration file defining the run."""
        return self.sbatch_contents

    def run(self) -> int | None:
        """Run the specified run configuration."""
        with NamedTemporaryFile(suffix=".sbatch", dir=Path("./"), mode="w+") as sbatch_tmp:
            sbatch_tmp.write(self.sbatch_contents)
            sbatch_tmp.flush()
            result = subprocess_run(
                ["sbatch", Path(sbatch_tmp.name)], check=True, stdout=subprocess_PIPE
            )  # noqa: S603
            job_id_search = re_search(JOB_ID_REGEX, result.stdout.decode("utf-8"))
            if job_id_search is None:
                return None
            return int(job_id_search.group(1))


def get_reference_impl(args: str) -> RunConfiguration:
    """Build a run configuration for the reference implementation"""
    run = RunConfiguration()
    run.add_sbatch_config({"cpus-per-task": "48"})
    run.add_module_loads(["GCC/11.3.0"])
    run.add_display_environment()
    run.add_build_step(["make -j 8"], directory=Path("../"))
    run.add_run_step("./test_HPCCG", args)
    return run


if __name__ == "__main__":
    for args in ["50 50 50"]:
        print(get_reference_impl(args))
