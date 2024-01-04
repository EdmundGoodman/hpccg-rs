#!/bin/sh

set -e

# Set up for the batch job
cd ..
cargo build --release
cd utils
mkdir -p joboutputs

# Submit the batch to Kudu, and record its batch number
BATCH=$( sbatch kudu.sbatch )
BATCHNO=$( echo $BATCH | sed 's/[^0-9]//g' )
echo "===== Job $BATCHNO has been submitted! ====="

# Show the job position in the queue
echo "===== My jobs ====="
echo "Note: Don't worry if the job is PENDING, the job will be ran as soon as possible."
squeue -u $( whoami ) -o "%.8i %.20j %.10T %.5M %.20R %.20e"

# Soft-link to the most recent run for convenience
rm -f most_recent.out most_recent.err
ln -s joboutputs/hpccg-rs_$BATCHNO.out most_recent.out
ln -s joboutputs/hpccg-rs_$BATCHNO.err most_recent.err

# Show the estimate to to complete
#ENDTIME=$( squeue -o "%i %e" | grep $BATCHNO | cut -d " " -f 2 )
#echo "===== Job $BATCHNO has been submitted, should be finished by $ENDTIME ====="
