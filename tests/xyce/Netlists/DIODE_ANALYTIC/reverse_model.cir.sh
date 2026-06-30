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

CIR1=reverse_model.cir
GOLDSCRIPT=./reverse_model.cir.gs.pl

rm -f $CIR1.prn $CIR1.err $CIR1.prn.gs $GOLDSCRIPT.err $CIR1.prn.out $CIR1.prn.out.err

$XYCE $CIR1 > /dev/null 2> $CIR1.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi

$GOLDSCRIPT > /dev/null 2> $GOLDSCRIPT.err
if [ "$?" -ne "0" ]; then
  echo "Exit code = 10"
  exit 10
fi

# Need to do a little massaging here to get xyce_verify to not complain.  
# Normally, goldstandard files and .prn files compare different *voltages*
# for the *same time*.  In this test, however, the goldstandard and .prn 
# files have the *same voltages* at *different times*.  The reason for this
# is that the analytical solution is written in an inverse manner; rather 
# than solving for vout as a function of t, we can only solve for t as a 
# function of vout.  xyce_verify has an interpolation algorithm to estimate 
# the Xyce-computed solution at the timepoints specified in the goldstandard
# file, but this only works so long as the last time step for the goldstandard
# file is larger than the last time step of the Xyce-computed file (can't 
# interpolate otherwise!).  
#
# I'm going to attempt to get around this in the following manner:  if the 
# goldstandard file ends before the Xyce-computed file, we'll switch the 
# order of arguments to xyce-verify.  Practically, this shouldn't make a 
# difference as to whether a given set of xyce-computed and goldstandard 
# file pairs pass or fail xyce-verify; if the two files are sufficiently 
# close, then the actual RMS percentage difference that's measured will, in
# general, be different when the order of the arguments is interchanged, but,
# so long as the error in one direction is well below the pass/fail threshold,
# the error in the other direction should be well below the threshold as well.
# If, on the other hand, the error in one direction is greater than the 
# pass/fail threshold, the RMS error can differ somewhat significantly from 
# one direction/ordering to the other, but, chances are that if the error is
# grossly off in one direction, it should be grossly off in the other direction
# as well.  It's possible, though seemingly highly unlikely, that the test 
# will barely pass for one ordering and barely fail for the other.


$XYCE_VERIFY  $CIR1 $CIR1.prn.gs $CIR1.prn > $CIR1.prn.out 2> $CIR1.prn.out.err

EXITCODE=$?
if [ "$EXITCODE" -ne "0" ]; then
    $XYCE_VERIFY $CIR1 $CIR1.prn $CIR1.prn.gs
else
    echo "Exit code = 0"
    exit 0
fi

EXITCODE=$?
if [ $EXITCODE -eq  0 ]; then
    echo "Exit code = 0"
    exit 0
else
    echo "Exit code = 2"
    exit 2
fi;
