#!/bin/sh

set -e

# Set up for the batch job
mkdir -p joboutputs
rm -f joboutputs/most_recent.out joboutputs/most_recent.err
CARGO_TARGET_DIR=./joboutputs/target cargo build --manifest-path=${1:-../Cargo.toml} --release

# Submit the batch to Kudu, and record its batch number
BATCH=$( sbatch kudu.sbatch )
BATCHNO=$( echo $BATCH | sed 's/[^0-9]//g' )
echo "===== Job $BATCHNO has been submitted! ====="

# Show the job position in the queue
echo "===== My jobs ====="
echo "Note: Don't worry if the job is PENDING, the job will be ran as soon as possible."
squeue -u $( whoami ) -o "%.8i %.20j %.10T %.5M %.20R %.20e"

# Soft-link to the most recent run for convenience
ln -s hpccg-rs_$BATCHNO.out ./joboutputs/most_recent.out
ln -s hpccg-rs_$BATCHNO.err ./joboutputs/most_recent.err

# Show the estimate to to complete
#ENDTIME=$( squeue -o "%i %e" | grep $BATCHNO | cut -d " " -f 2 )
#echo "===== Job $BATCHNO has been submitted, should be finished by $ENDTIME ====="
