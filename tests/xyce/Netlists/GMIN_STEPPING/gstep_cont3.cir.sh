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

CIR1=gstep.cir
CIR2=gstep_cont3a.cir
CIR3=gstep_cont3b.cir
CIR4=gstep_cont3c.cir


$XYCE $CIR1 > /dev/null 2> $CIR1.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi

$XYCE $CIR2 > /dev/null 2> $CIR2.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi

$XYCE $CIR3 > /dev/null 2> $CIR3.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi

$XYCE $CIR4 > /dev/null 2> $CIR4.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi

grep -v "Index" $CIR1.HOMOTOPY.prn | grep -v "End" > $CIR1.prn2
grep -v "Index" $CIR2.HOMOTOPY.prn | grep -v "End" > $CIR2.prn2
grep -v "Index" $CIR3.HOMOTOPY.prn | grep -v "End" > $CIR3.prn2
grep -v "Index" $CIR4.HOMOTOPY.prn | grep -v "End" > $CIR4.prn2

diff $CIR1.prn2 $CIR2.prn2 > $CIR2.prn.out 2> $CIR2.prn.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 2"
  exit 2
fi

diff $CIR1.prn2 $CIR3.prn2 > $CIR3.prn.out 2> $CIR3.prn.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 2"
  exit 2
fi

diff $CIR1.prn2 $CIR4.prn2 > $CIR4.prn.out 2> $CIR4.prn.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 2"
  exit 2
fi

# This test isn't really about the non-HOMOTOPY *prn files, but 
# compare them anyway.
#$XYCE_VERIFY $CIR1 $CIR2.prn $CIR1.prn  

EXITCODE=$?
if [ $EXITCODE -eq  0 ]; then
    echo "Exit code = 0"
    exit 0
else
    echo "Exit code = 2"
    exit 2
fi;

