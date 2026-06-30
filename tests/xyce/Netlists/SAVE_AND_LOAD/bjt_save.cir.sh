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
XYCE_VERIFY=$2
#XYCE_COMPARE=$3
#CIRFILE=$4
#GOLDPRN=$5

CIR1=bjt_save.cir
CIR2=bjt_load.cir
CIR3=bjt_baseline.cir
ICFILE=bjt.ic

#echo "";
#echo "    $CIR1...................."
$XYCE $CIR1 > $CIR1.out 2> $CIR1.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi
#echo "    $CIR2...................."
$XYCE $CIR2 > $CIR2.out 2> $CIR2.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi
#echo "    $CIR3...................."
$XYCE $CIR3 > $CIR3.out 2> $CIR3.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi

# grep that the .ic file actually uses .NODESET
grep "IC" $ICFILE > /dev/null 2>&1
if [ "$?" -ne "0" ]; then
  echo ".ic file did not use .IC"
  echo "Exit code = 2"
  exit 2
fi

# This test isn't really about the non-HOMOTOPY *prn files, but 
# compare them anyway.
#$XYCE_VERIFY --verbose $CIR3 $CIR2.prn $CIR3.prn  
$XYCE_VERIFY $CIR3 $CIR2.prn $CIR3.prn  

EXITCODE=$?
if [ $EXITCODE -eq  0 ]; then
    echo "Exit code = 0"
    exit 0
else
    echo "Exit code = 2"
    exit 2
fi;
