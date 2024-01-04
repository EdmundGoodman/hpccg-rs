#!/bin/bash
#
#SBATCH --job-name=multicore-cpu # Job name for tracking
#SBATCH --partition=cpu-batch  # Partition you wish to use (see above for list)
#SBATCH --cpus-per-task=40     # Number of CPU threads used by your job, set this upto 40 as required.
#SBATCH --mem=60000            # 60GB RAM
#SBATCH --exclusive=mcs        # Exclusive mode, only this job will run
#SBATCH --time=2-00:00:00      # Job time limit set to 2 days (48 hours)
#
#SBATCH --mail-type=TIME_LIMIT_80 # Events to send email on, remove if you don't want this
#SBATCH --output=joboutputs/%j/hpccg-rs_%j.out # Standard out from your job
#SBATCH --error=joboutputs/%j/hpccg-rs_%j.err  # Standard error from your job

## Initialisation ##
source /etc/profile.d/modules.sh
mkdir -p joboutputs/%j

## Execute your program(s) ##
perf record -g ../target/release/hpccg-rs
mv perf.data joboutputs/%j/perf_%j.data