#!/bin/sh

if [ $# -lt 1 ]; then
	echo "Please provide a .sbatch file and any additional flags for the 'sbatch' command"
	exit 1
fi

echo "===== Running 'sbatch $@' ====="
BATCH=$( sbatch $@ )
BATCHNO=$( echo $BATCH | sed 's/[^0-9]//g' )
mkdir $BATCHNO
echo "===== Job $BATCHNO submitted! Current queus state: ====="
squeue -u $( whoami ) -o "%.8i %.20j %.10T %.5M %.20R %.20e"
