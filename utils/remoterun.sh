#!/bin/sh

# This script requires at lease a singular parameter, an executable. which
# should be the first parameter. Any further parameters should be the ones
# required by the executable itself, as these will get passed to the program
# when ran by the compute node.

if [ $# -lt 1 ]; then
	echo "Please provide a command to run on the batch computer system"
	exit 1
fi

echo "#!/bin/sh

#SBATCH --job-name=multicore-cpu
#SBATCH --partition=cpu-batch
#SBATCH --cpus-per-task=40
#SBATCH --time=10:00
#SBATCH --mem=60000
#SBATCH --exclusive=mcs
#SBATCH --output=%j/cs257_output_%j.out
#SBATCH --error=%j/cs257_error_%j.err
echo ===== ENVIRONMENT =====
. /etc/profile.d/modules.sh
lscpu

$@
" > tmp

BATCH=$( sbatch tmp )
BATCHNO=$( echo $BATCH | sed 's/[^0-9]//g' )

rm tmp

mkdir $BATCHNO

echo "===== Job $BATCHNO has been submitted! ====="

echo "===== My jobs ====="
echo "Note: Don't worry if the job is PENDING, the job will be ran as soon as possible."
squeue -u $( whoami ) -o "%.8i %.20j %.10T %.5M %.20R %.20e"
