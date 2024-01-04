#!/bin/sh

set -e

# Compile the case to run
cargo build --release

# Submit the batch to Kudu, and record its batch number
BATCH=$( sbatch kudu.sbatch )
BATCHNO=$( echo $BATCH | sed 's/[^0-9]//g' )
echo "===== Job $BATCHNO has been submitted! ====="

# Show the job position in the queue
echo "===== My jobs ====="
echo "Note: Don't worry if the job is PENDING, the job will be ran as soon as possible."
squeue -u $( whoami ) -o "%.8i %.20j %.10T %.5M %.20R %.20e"

# Show the estimate to to complete
ENDTIME=$( squeue -o "%i %e" | grep $BATCHNO | cut -d " " -f 2 )
echo "===== Job $BATCHNO has been submitted, should be finished by $ENDTIME ====="
