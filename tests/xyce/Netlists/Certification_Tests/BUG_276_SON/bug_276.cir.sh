#!/bin/sh

# The input arguments to this script are:
# $1 = location of Xyce binary
# $2 = location of xyce_verify.pl script
# $3 = location of compare script 
# $4 = location of circuit file to test
# $5 = location of gold standard prn file

# If Xyce does not produce a prn file, then we return exit code 10.
# If Xyce succeeds, but the test fails, then we return exit code 2.
# If the shell script fails for some reason, then we return exit code 1.
# Otherwise we return the exit code of compare or xyce_verify.pl

# Since the shell script runs Xyce also, it is responsible for capturing any
# error output from Xyce.  The script run_xyce_regression captures the test
# output and handles the resulting files.  

XYCE=$1
#XYCE_VERIFY=$2
#XYCE_COMPARE=$3
#CIRFILE=$4
#GOLDPRN=$5

CIR1=bug_276.cir
rm -f $CIR1.prn $CIR1.res $CIR1.err $CIR1.out

$XYCE $CIR1 > $CIR1.out 2> $CIR1.err
RETCODE=$?
#echo "return code from Xyce = $RETCODE"

# In this test, a return code of 255 is a success.   The circuit will fail
# several tests.  This test is to make sure that when it does fail,
# it correctly returns an error.

if [ "$RETCODE" -ne 0 ]; then
#success, circuit correctly failed
  echo "Exit code = 0"
  exit 0
else
# failure, because circuit should not have succeeded
  echo "Exit code = 2"
  exit 10
fi

