#!/bin/sh

# The input arguments to this script are set up in 
# Xyce_Test/TestScripts/tsc_run_test_suite and are as follows:
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
XYCE_VERIFY=$2
#XYCE_COMPARE=$3
CIRFILE=$4
GOLDPRN=$5

$XYCE $CIRFILE > $CIRFILE.out 2> $CIRFILE.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi

$XYCE_VERIFY --goodres=../../OutputData/BUG_616/$CIRFILE.res --testres=$CIRFILE.res $CIRFILE $GOLDPRN $CIRFILE.prn > $CIRFILE.prn.out 2> $CIRFILE.prn.err 
RETVAL=$?
if [ "$RETVAL" -ne "0" ]; then
  echo "xyce_verify exit code = $RETVAL"
  echo "Exit code = 2"
  exit 2
fi
echo "Exit code = $RETVAL"
exit $RETVAL

